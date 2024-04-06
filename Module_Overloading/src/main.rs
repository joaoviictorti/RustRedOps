#![allow(unused_assignments)]

mod utils;
mod cmd;

use std::{ffi::c_void, mem::size_of, ptr::null_mut};
use clap::Parser;
use cmd::Args;
use ntapi::ntmmapi::{NtMapViewOfSection, ViewShare};
use utils::{get_peb, image_ordinal, image_snap_by_ordinal, Dll, Exe, BASE_RELOCATION_ENTRY, PE};
use windows::core::PCSTR;
use windows::Wdk::Storage::FileSystem::NtCreateSection;
use windows::Win32::Foundation::{FARPROC, GENERIC_READ, HANDLE, HINSTANCE, STATUS_SUCCESS};
use windows::Win32::Storage::FileSystem::{FILE_ATTRIBUTE_NORMAL, FILE_SHARE_MODE, OPEN_EXISTING, CreateFileA};
use windows::Win32::System::{
    Memory::*,
    SystemServices::*,
    Diagnostics::Debug::*,
    LibraryLoader::{LoadLibraryA, GetProcAddress},
    WindowsProgramming::IMAGE_THUNK_DATA64,
    Threading::RTL_USER_PROCESS_PARAMETERS
};

fn main() -> Result<(), String> {
    let args = Args::parse();

    let buffer = std::fs::read(&args.file).map_err(|e| format!("[!] Error reading file: {}", e))?;
    let mut pe = initialize_pe(buffer)?;
    let module_dll  = load_dll(args.dll)?;
    load_exe(&mut pe, module_dll, args.args.as_deref().unwrap_or(""))?;

    Ok(())
}

fn load_exe(pe: &mut PE, module_dll: *mut c_void, args: &str) -> Result<(), String>{
    let address = unsafe {
        VirtualAlloc(
            None,
            (*pe.nt_header).OptionalHeader.SizeOfImage as usize,
            MEM_COMMIT | MEM_RESERVE,
            PAGE_READWRITE,
        )
    };

    if address.is_null() {
        return Err("VirtualAlloc Failed".to_string());
    }

    let mut tmp_section = pe.section_header;

    unsafe {
        for _ in 0..(*pe.nt_header).FileHeader.NumberOfSections {
            let dst = (*tmp_section).VirtualAddress as isize;
            let src_start = (*tmp_section).PointerToRawData as usize;
            let src_end = src_start + (*tmp_section).SizeOfRawData as usize;
        
            if src_end <= pe.file_buffer.len() {
                let src = &pe.file_buffer[src_start..src_end];
                std::ptr::copy_nonoverlapping(
                    src.as_ptr(),
                    address.offset(dst) as _,
                    src.len(),
                );

            } else {
                return Err("Section outside the buffer limits".to_string());
            }

            tmp_section = (tmp_section as usize + size_of::<IMAGE_SECTION_HEADER>()) as *mut IMAGE_SECTION_HEADER;
        }
    }

    fixing_iat(pe, address)?;

    let mut old_protect = PAGE_PROTECTION_FLAGS(0);
    unsafe { 
        VirtualProtect(
            module_dll, 
            (*pe.nt_header).OptionalHeader.SizeOfImage as usize, 
            PAGE_READWRITE, 
            &mut old_protect
        ).map_err(|e| format!("[!] VirtualProtect Failed With Status: {e}"))?; 
    };

    unsafe { std::ptr::copy_nonoverlapping(address, module_dll, (*pe.nt_header).OptionalHeader.SizeOfImage as usize) };

    realoc_data(pe, module_dll)?;

    unsafe { 
        VirtualProtect(
            module_dll,  
            (*pe.nt_header).OptionalHeader.SizeOfHeaders as usize, 
            PAGE_READONLY, 
            &mut old_protect
        ).map_err(|e| format!("[!] VirtualProtect (2) Failed With Status: {e}"))?; 
    };

    fixing_arguments(args);

    fixing_memory(pe, module_dll)?;

    let entrypoint = unsafe { (module_dll as usize + (*pe.nt_header).OptionalHeader.AddressOfEntryPoint as usize) as *mut c_void };
    
    unsafe {
        if pe.is_dll {
            let func_exe =  std::mem::transmute::<_, Dll>(entrypoint);
            func_exe(HINSTANCE(address as isize), DLL_PROCESS_ATTACH, null_mut());
        } else {
            let func_dll =  std::mem::transmute::<_, Exe>(entrypoint);
            func_dll();
        }

    };

    Ok(())
}


///
/// Initializing the PE headers of the next target level
///
fn initialize_pe(buffer: Vec<u8>) -> Result<PE, String> {
    unsafe {
        let dos_header = buffer.as_ptr() as *mut IMAGE_DOS_HEADER;
        if (*dos_header).e_magic != IMAGE_DOS_SIGNATURE {
            return Err("Invalid DOS SIGNATURE".to_string());
        }

        let nt_header = (dos_header as usize + (*dos_header).e_lfanew as usize) as *mut IMAGE_NT_HEADERS64;
        if (*nt_header).Signature != IMAGE_NT_SIGNATURE {
            return Err("INVALID NT SIGNATURE".to_string());
        }

        let mut section_header = (nt_header as usize + size_of::<IMAGE_NT_HEADERS64>()) as *mut IMAGE_SECTION_HEADER;
        for i in 0..(*nt_header).FileHeader.NumberOfSections {
            let section = (*section_header.add(i.into())).Name;
            let name = String::from_utf8(section.to_vec()).expect("Error reading section");
            let name = name.trim_matches('\0');
            if name == ".text" {
                break;
            }
            section_header = (section_header as usize + size_of::<IMAGE_SECTION_HEADER>()) as *mut IMAGE_SECTION_HEADER;
        }

        let pe = PE {
            file_buffer : buffer,
            nt_header : nt_header as *mut IMAGE_NT_HEADERS64,
            section_header,
            is_dll : (*nt_header).FileHeader.Characteristics.0 & IMAGE_FILE_DLL.0 != 0,
            entry_import_data : (*nt_header).OptionalHeader.DataDirectory[IMAGE_DIRECTORY_ENTRY_IMPORT.0 as usize],
            entry_basereloc_data : (*nt_header).OptionalHeader.DataDirectory[IMAGE_DIRECTORY_ENTRY_BASERELOC.0 as usize],
        };

        Ok(pe)
    }


}

///
/// Map the DLL to the process
/// 
fn load_dll(dll: String) -> Result<*mut c_void, String> {
    unsafe {
        let dll = std::ffi::CString::new(dll).unwrap().into_raw();
        let h_file = CreateFileA(
            PCSTR(dll as _),
            GENERIC_READ.0,
            FILE_SHARE_MODE(0),
            None,
            OPEN_EXISTING,
            FILE_ATTRIBUTE_NORMAL,
            None,
        );

        if h_file.is_err() {
            return Err(format!("CreateFileA Failed With Status: {:?}", h_file.err()))
        }

        let mut section = HANDLE::default();
        let status = NtCreateSection(
            &mut section,
            SECTION_ALL_ACCESS.0,
            None,
            None,
            PAGE_READONLY.0,
            SEC_IMAGE.0,
            h_file.unwrap(),
        );

        if status != STATUS_SUCCESS {
            return Err(format!("NtCreateSection Failed With Status: {:?}", status))
        }

        let mut mapped_module: *mut ntapi::winapi::ctypes::c_void = null_mut();
        let mut view_size = 0;
        let status = NtMapViewOfSection(
            section.0 as _,
            0xffffffffffffffffu64 as _,
            &mut mapped_module,
            0,
            0,
            null_mut(),
            &mut view_size,
            ViewShare,
            0,
            PAGE_EXECUTE_READWRITE.0,
        );

        if status != 0 {
            return Err(format!("NtMapViewOfSection Failed With Status {}", status));
        }

        let dos_header = mapped_module as *mut IMAGE_DOS_HEADER;
        let nt_header = (mapped_module as usize + (*dos_header).e_lfanew as usize) as *mut IMAGE_NT_HEADERS64;
        if (*nt_header).Signature != IMAGE_NT_SIGNATURE {
            return Err("IMAGE SIGNATURE INVALID".to_string());
        }

        Ok(mapped_module as *mut c_void)
    }
}

///
/// Create the PE address relationship
/// 
fn realoc_data(pe: &mut PE, address: *mut c_void) -> Result<(), String> {
    unsafe {
        let mut base_relocation = address.offset(pe.entry_basereloc_data.VirtualAddress as isize) as *mut IMAGE_BASE_RELOCATION;
        let offset = address.wrapping_sub((*pe.nt_header).OptionalHeader.ImageBase as usize);
        while (*base_relocation).VirtualAddress != 0 {
            let mut base_entry = base_relocation.offset(1) as *mut BASE_RELOCATION_ENTRY;
            let block_end = (base_relocation as *mut u8).offset((*base_relocation).SizeOfBlock as isize) as *mut BASE_RELOCATION_ENTRY;

            while base_entry < block_end {
                let entry = *base_entry;
                let entry_type = entry.type_();
                let entry_offset = entry.offset() as u32;
                let target_address = address.wrapping_add(((*base_relocation).VirtualAddress + entry_offset) as usize);

                match entry_type as u32 {
                    IMAGE_REL_BASED_DIR64 => {
                        let patch_address = target_address as *mut isize;
                        *patch_address += offset as isize;
                    }
                    IMAGE_REL_BASED_HIGHLOW => {
                        let patch_address = target_address as *mut u32;
                        *patch_address = patch_address.read().wrapping_add(offset as u32);
                    }
                    IMAGE_REL_BASED_HIGH => {
                        let patch_address = target_address as *mut u16;
                        let high = (*patch_address as u32).wrapping_add((offset as u32 >> 16) & 0xFFFF);
                        *patch_address = high as u16
                    }
                    IMAGE_REL_BASED_LOW => {
                        let patch_address = target_address as *mut u16;
                        let low = (*patch_address as u32).wrapping_add(offset as u32 & 0xFFFF);
                        *patch_address = low as u16;
                    }
                    IMAGE_REL_BASED_ABSOLUTE => {}
                    _ => {
                        return Err("Unknown relocation type".to_string());
                    }
                }

                base_entry = base_entry.offset(1);
            }

            base_relocation = base_entry as *mut IMAGE_BASE_RELOCATION;
        }
    }

    Ok(())
}

///
/// Solving the IAT by loading the DLLs into the process and then filling the IAT with their functions.
/// 
fn fixing_iat(pe: &PE, address: *mut c_void) -> Result<(), String> {
    unsafe {
        let entries = (pe.entry_import_data.Size as usize / size_of::<IMAGE_IMPORT_DESCRIPTOR>()) as u32;
        let img_import_descriptor = address.offset(pe.entry_import_data.VirtualAddress as isize) as *mut IMAGE_IMPORT_DESCRIPTOR;

        for i in 0..entries {
            let img_import_descriptor = img_import_descriptor.offset(i as isize);
            let original_first_chunk_rva = (*img_import_descriptor).Anonymous.OriginalFirstThunk;
            let first_thunk_rva = (*img_import_descriptor).FirstThunk;
            if original_first_chunk_rva == 0 && first_thunk_rva == 0 {
                break;
            }

            let dll_name = address.offset((*img_import_descriptor).Name as isize) as *const i8;
            let mut thunk_size = 0;
            let h_module = LoadLibraryA(PCSTR(dll_name as _)).unwrap_or_else(|e| panic!("LoadLibrary Failed With Status: {}", e));

            loop {
                let original_first_chunk = address.offset(original_first_chunk_rva as isize + thunk_size) as *mut IMAGE_THUNK_DATA64;
                let first_thunk = address.offset(first_thunk_rva as isize + thunk_size) as *mut IMAGE_THUNK_DATA64;
                if (*original_first_chunk).u1.Function == 0 && (*first_thunk).u1.Function == 0  {
                    break;
                }

                let mut func_address: FARPROC = Default::default();
                let mut name: *const i8 = null_mut();

                if image_snap_by_ordinal((*original_first_chunk).u1.Ordinal) {
                    let ordinal = image_ordinal((*original_first_chunk).u1.Ordinal);
                    func_address = GetProcAddress(h_module, PCSTR(ordinal as *const u8));
                } else {
                    let image_import_name = address.offset((*original_first_chunk).u1.AddressOfData as isize) as *mut IMAGE_IMPORT_BY_NAME;
                    name = &(*image_import_name).Name as *const i8;
                    func_address = GetProcAddress(h_module, PCSTR(name as *const u8));
                }

                match func_address {
                    Some(f) => {
                        (*first_thunk).u1.Function =  f as *const () as u64;
                    },
                    None => {
                        return Err(format!("The expected function was not found: {:?}", name));
                    }
                }

                thunk_size += size_of::<IMAGE_THUNK_DATA64>() as isize;
            }
        }
    }

    Ok(())
}

///
/// Defining memory permissions for each section.
/// 
fn fixing_memory(pe: &mut PE, address: *mut c_void) -> Result<(), String> {
    unsafe {
        for _ in 0..(*pe.nt_header).FileHeader.NumberOfSections {
            let mut protection = PAGE_PROTECTION_FLAGS(0);
            let image_section_characteristics = IMAGE_SECTION_CHARACTERISTICS(0);
            if (*pe.section_header).SizeOfRawData == 0 || (*pe.section_header).VirtualAddress == 0 {
                continue;
            }

            if (*pe.section_header).Characteristics & IMAGE_SCN_MEM_WRITE != image_section_characteristics {
                protection = PAGE_WRITECOPY
            }

            if (*pe.section_header).Characteristics & IMAGE_SCN_MEM_READ != image_section_characteristics {
                protection = PAGE_READONLY
            }

            if (*pe.section_header).Characteristics & IMAGE_SCN_MEM_WRITE != image_section_characteristics
                && (*pe.section_header).Characteristics & IMAGE_SCN_MEM_READ != image_section_characteristics {
                protection = PAGE_READWRITE
            }

            if (*pe.section_header).Characteristics & IMAGE_SCN_MEM_EXECUTE != image_section_characteristics {
                protection = PAGE_EXECUTE
            }

            if (*pe.section_header).Characteristics & IMAGE_SCN_MEM_EXECUTE != image_section_characteristics
                && (*pe.section_header).Characteristics & IMAGE_SCN_MEM_WRITE != image_section_characteristics {
                protection = PAGE_EXECUTE_WRITECOPY
            }

            if (*pe.section_header).Characteristics & IMAGE_SCN_MEM_EXECUTE != image_section_characteristics
                && (*pe.section_header).Characteristics & IMAGE_SCN_MEM_READ != image_section_characteristics {
                    protection = PAGE_EXECUTE_READ
            }

            if (*pe.section_header).Characteristics & IMAGE_SCN_MEM_EXECUTE != image_section_characteristics
                && (*pe.section_header).Characteristics & IMAGE_SCN_MEM_WRITE != image_section_characteristics
                && (*pe.section_header).Characteristics & IMAGE_SCN_MEM_READ != image_section_characteristics {
                    protection = PAGE_EXECUTE_READWRITE;
            }

            let mut old_protect = PAGE_PROTECTION_FLAGS(0);
            VirtualProtect(
                address.offset((*pe.section_header).VirtualAddress as isize),
                (*pe.section_header).SizeOfRawData as usize,
                protection,
                &mut old_protect,
            ).map_err(|e| format!("VirtualProtect (3) Failed With Status: {e}"))?;

            pe.section_header = (pe.section_header as usize + size_of::<IMAGE_SECTION_HEADER>()) as *mut IMAGE_SECTION_HEADER;
        }
    }

    Ok(())
}

///
/// Readjust the arguments to be passed to the target binary (If necessary)
/// 
fn fixing_arguments(args: &str) {
    let peb = unsafe { get_peb() };
    let process_parameters = unsafe { (*peb).ProcessParameters as *mut RTL_USER_PROCESS_PARAMETERS };
    unsafe {
        std::ptr::write_bytes((*process_parameters).CommandLine.Buffer.0, 0, (*process_parameters).CommandLine.Length as usize);

        let current_exe = std::env::current_exe().unwrap();
        let path_name: Vec<u16> = format!("\"{}\" {}\0", current_exe.to_string_lossy(), args)
            .encode_utf16()
            .collect();

        std::ptr::copy_nonoverlapping(path_name.as_ptr(), (*process_parameters).CommandLine.Buffer.0, path_name.len());
        (*process_parameters).CommandLine.Length = (path_name.len() * 2) as u16;
        (*process_parameters).CommandLine.MaximumLength = (path_name.len() * 2) as u16;
    }
}