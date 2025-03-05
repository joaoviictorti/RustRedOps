#![allow(unused_assignments)]

mod utils;
mod cmd;

use std::{
    ffi::{c_void, CStr}, 
    mem::size_of, 
    ptr::null_mut
};
use {
    cmd::Args,
    clap::Parser, 
    utils::{
        PE, Dll, Exe,
        BASE_RELOCATION_ENTRY,
        get_peb, image_ordinal, 
        image_snap_by_ordinal,
    }, 
};

use ntapi::ntmmapi::{NtMapViewOfSection, ViewShare};
use windows::{
    core::PCSTR, 
    Wdk::Storage::FileSystem::NtCreateSection, 
    Win32::{
        Foundation::{
            GetLastError, GENERIC_READ, HANDLE, 
            HINSTANCE, STATUS_SUCCESS
        }, 
        Storage::FileSystem::{
            CreateFileA, FILE_ATTRIBUTE_NORMAL, 
            FILE_SHARE_MODE, OPEN_EXISTING
        }, 
        System::{
            Memory::*, 
            SystemServices::*,
            Diagnostics::Debug::*,
            Threading::RTL_USER_PROCESS_PARAMETERS, 
            WindowsProgramming::IMAGE_THUNK_DATA64,
            LibraryLoader::{GetProcAddress, LoadLibraryA}, 
        }
    }
};

fn main() -> Result<(), String> {
    let arguments = Args::parse();
    let buffer = std::fs::read(&arguments.file)
        .map_err(|e| format!("[!] Error reading file: {}", e))?;
    
    let mut pe = init_pe(buffer)?;
    let module_dll  = load_dll(arguments.dll)?;
    load_exe(&mut pe, module_dll, arguments.args.as_deref().unwrap_or(""))?;

    Ok(())
}

/// Loads the executable into memory and prepares it for execution.
///
/// # Arguments
///
/// * `pe` - A mutable reference to a PE struct representing the loaded PE file.
/// * `module_dll` - A pointer to the loaded DLL module.
/// * `args` - Command line arguments to be passed to the executable.
///
/// # Returns
///
/// A `Result` which is `Ok` if the executable is successfully loaded and `Err` if there is an error.
fn load_exe(pe: &mut PE, module_dll: *mut c_void, args: &str) -> Result<(), String>{
    unsafe {
        let address = VirtualAlloc(
            None,
            (*pe.nt_header).OptionalHeader.SizeOfImage as usize,
            MEM_COMMIT | MEM_RESERVE,
            PAGE_READWRITE,
        );
    
        if address.is_null() {
            return Err(format!("VirtualAlloc Failed With Error: {:?}", GetLastError()));
        }
    
        let mut tmp_section = pe.section_header;
        for _ in 0..(*pe.nt_header).FileHeader.NumberOfSections {
            let dst = (*tmp_section).VirtualAddress as isize;
            let start = (*tmp_section).PointerToRawData as usize;
            let end = start + (*tmp_section).SizeOfRawData as usize;
        
            if end <= pe.file_buffer.len() {
                let src = &pe.file_buffer[start..end];
                std::ptr::copy_nonoverlapping(
                    src.as_ptr(),
                    address.offset(dst).cast(),
                    src.len(),
                );

            } else {
                return Err(String::from("Section outside the buffer limits"));
            }

            tmp_section = tmp_section.add(1);
        }
    
        let mut old_protect = PAGE_PROTECTION_FLAGS(0);
        VirtualProtect(
            module_dll, 
            (*pe.nt_header).OptionalHeader.SizeOfImage as usize, 
            PAGE_READWRITE, 
            &mut old_protect
        ).map_err(|e| format!("[!] VirtualProtect Failed With Status: {e}"))?; 
    
       std::ptr::copy_nonoverlapping(address, module_dll, (*pe.nt_header).OptionalHeader.SizeOfImage as usize);
    
        // Adjusting the IAT header
        fixing_iat(pe, module_dll)?;

        // Adjusting relocations
        realoc_data(pe, module_dll)?;
    
        VirtualProtect(
            module_dll,  
            (*pe.nt_header).OptionalHeader.SizeOfHeaders as usize, 
            PAGE_READONLY, 
            &mut old_protect
        ).map_err(|e| format!("[!] VirtualProtect [{}] Failed With Status: {e}", line!()))?; 
    
        // Adjusting the arguments (if any)
        fixing_arguments(args)?;
    
        // Adjusting memory permissions
        fixing_memory(pe, module_dll)?;
    
        let entrypoint = (module_dll as usize + (*pe.nt_header).OptionalHeader.AddressOfEntryPoint as usize) as *mut c_void;
        if pe.is_dll {
            let func_dll =  std::mem::transmute::<_, Dll>(entrypoint);
            func_dll(HINSTANCE(address as isize), DLL_PROCESS_ATTACH, null_mut());
        } else {
            let func_exe =  std::mem::transmute::<_, Exe>(entrypoint);
            func_exe();
        }
    }

    Ok(())
}

/// Initializes the PE structure by reading and parsing the PE headers from the given buffer.
///
/// # Arguments
///
/// * `buffer` - A vector of bytes representing the PE file.
///
/// # Returns
///
/// * A `Result` containing a `PE` struct if the initialization is successful, or a `String` error message if it fails.
fn init_pe(buffer: Vec<u8>) -> Result<PE, String> {
    unsafe {
        let dos_header = buffer.as_ptr() as *mut IMAGE_DOS_HEADER;
        if (*dos_header).e_magic != IMAGE_DOS_SIGNATURE {
            return Err(String::from("Invalid DOS SIGNATURE"));
        }

        let nt_header = (dos_header as usize + (*dos_header).e_lfanew as usize) as *mut IMAGE_NT_HEADERS64;
        if (*nt_header).Signature != IMAGE_NT_SIGNATURE {
            return Err(String::from("INVALID NT SIGNATURE"));
        }

        let mut section_header = (nt_header as usize + size_of::<IMAGE_NT_HEADERS64>()) as *mut IMAGE_SECTION_HEADER;
        for i in 0..(*nt_header).FileHeader.NumberOfSections {
            let section = (*section_header.add(i.into())).Name;
            let name = String::from_utf8_lossy(&section);
            if name.trim_matches('\0') == ".text" {
                break;
            }

            section_header = section_header.add(1);
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

/// Loads a DLL file into the process memory.
///
/// # Arguments
///
/// * `dll` - A string representing the path to the DLL file.
///
/// # Returns
///
/// A `Result` containing a pointer to the loaded DLL module if successful, or a `String` error message if it fails.
fn load_dll(dll: String) -> Result<*mut c_void, String> {
    unsafe {
        let dll = std::ffi::CString::new(dll).unwrap().into_raw();
        let h_file = CreateFileA(
            PCSTR(dll.cast()),
            GENERIC_READ.0,
            FILE_SHARE_MODE(0),
            None,
            OPEN_EXISTING,
            FILE_ATTRIBUTE_NORMAL,
            None,
        ).map_err(|e| format!("CreateFileA Failed With Error: {e}"))?;

        let mut section = HANDLE::default();
        let status = NtCreateSection(
            &mut section,
            SECTION_ALL_ACCESS.0,
            None,
            None,
            PAGE_READONLY.0,
            SEC_IMAGE.0,
            h_file,
        );

        if status != STATUS_SUCCESS {
            return Err(format!("NtCreateSection Failed With Status: {:?}", status))
        }

        let mut mapped_module = null_mut();
        let mut view_size = 0;
        let status = NtMapViewOfSection(
            section.0 as _,
            -1isize as *mut ntapi::winapi::ctypes::c_void ,
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
            return Err(format!("NtMapViewOfSection Failed With Status {status}"));
        }

        let dos_header = mapped_module as *mut IMAGE_DOS_HEADER;
        let nt_header = (mapped_module as usize + (*dos_header).e_lfanew as usize) as *mut IMAGE_NT_HEADERS64;
        if (*nt_header).Signature != IMAGE_NT_SIGNATURE {
            return Err(String::from("IMAGE SIGNATURE INVALID"));
        }

        Ok(mapped_module as *mut c_void)
    }
}

/// Adjusts the base relocations of the PE file to the new address.
///
/// # Arguments
///
/// * `pe` - A mutable reference to a PE struct representing the loaded PE file.
/// * `address` - The base address of the allocated memory for the PE file.
///
/// # Returns
///
/// A `Result` which is `Ok` if the relocations are successfully adjusted, or `Err` if there is an error.
fn realoc_data(pe: &mut PE, address: *mut c_void) -> Result<(), String> {
    unsafe {
        let offset = address.wrapping_sub((*pe.nt_header).OptionalHeader.ImageBase as usize);
        let mut base_relocation = address.offset(pe.entry_basereloc_data.VirtualAddress as isize) as *mut IMAGE_BASE_RELOCATION;

        while (*base_relocation).VirtualAddress != 0 {
            let mut base_entry = base_relocation.offset(1) as *mut BASE_RELOCATION_ENTRY;
            let block_end = (base_relocation as *mut u8).offset((*base_relocation).SizeOfBlock as isize) as *mut BASE_RELOCATION_ENTRY;

            while base_entry < block_end {
                let entry = *base_entry;
                let entry_type = entry.type_();
                let entry_offset = entry.offset() as u32;
                let target_address = address.add(((*base_relocation).VirtualAddress + entry_offset) as usize);

                match entry_type as u32 {
                    IMAGE_REL_BASED_DIR64 => *(target_address as *mut isize) += offset as isize,
                    IMAGE_REL_BASED_HIGHLOW => *(target_address as *mut u32) = (*(target_address as *mut u32)).wrapping_add(offset as u32),
                    IMAGE_REL_BASED_HIGH => *(target_address as *mut u16) = (*(target_address as *mut u16) as u32).wrapping_add((offset as u32 >> 16) & 0xFFFF) as u16,
                    IMAGE_REL_BASED_LOW => *(target_address as *mut u16) = (*(target_address as *mut u16) as u32).wrapping_add(offset as u32 & 0xFFFF) as u16,
                    IMAGE_REL_BASED_ABSOLUTE => {}, // No relocation needed
                    _ => return Err(format!("Unknown relocation type: {}", entry_type))
                }

                base_entry = base_entry.add(1);
            }

            base_relocation = base_entry as *mut IMAGE_BASE_RELOCATION;
        }
    }

    Ok(())
}

/// Fixes the Import Address Table (IAT) by loading the required DLLs and resolving the function addresses.
///
/// # Arguments
///
/// * `pe` - A reference to a PE struct representing the loaded PE file.
/// * `address` - The base address of the allocated memory for the PE file.
///
/// # Returns
///
/// * A `Result` which is `Ok` if the IAT is successfully fixed, or `Err` if there is an error.
fn fixing_iat(pe: &PE, address: *mut c_void) -> Result<(), String> {
    unsafe {
        let entries = (pe.entry_import_data.Size as usize / size_of::<IMAGE_IMPORT_DESCRIPTOR>()) as u32;
        let img_import_descriptor = address.offset(pe.entry_import_data.VirtualAddress as isize) 
            as *mut IMAGE_IMPORT_DESCRIPTOR;

        for i in 0..entries {
            let img_import_descriptor = img_import_descriptor.offset(i as isize);
            let original_first_chunk_rva = (*img_import_descriptor).Anonymous.OriginalFirstThunk;
            let first_thunk_rva = (*img_import_descriptor).FirstThunk;
            if original_first_chunk_rva == 0 && first_thunk_rva == 0 {
                break;
            }

            let dll_name = address.offset((*img_import_descriptor).Name as isize) as *const i8;
            let mut thunk_size = 0;
            let h_module = LoadLibraryA(PCSTR(dll_name as _))
                .map_err(|e| format!("LoadLibrary Failed With Status: {e}"))?;

            loop {
                let original_first_chunk = address.offset(original_first_chunk_rva as isize + thunk_size) as *mut IMAGE_THUNK_DATA64;
                let first_thunk = address.offset(first_thunk_rva as isize + thunk_size) as *mut IMAGE_THUNK_DATA64;
                if (*original_first_chunk).u1.Function == 0 && (*first_thunk).u1.Function == 0  {
                    break;
                }

                let func_address = if image_snap_by_ordinal((*original_first_chunk).u1.Ordinal) {
                    // Resolve function by ordinal
                    let ordinal = image_ordinal((*original_first_chunk).u1.Ordinal);
                    GetProcAddress(h_module, PCSTR(ordinal as *const u8))
                } else {
                    // Resolve function by name
                    let import_by_name = address.add((*original_first_chunk).u1.AddressOfData as usize) as *mut IMAGE_IMPORT_BY_NAME;
                    let func_name = &(*import_by_name).Name as *const i8;
                    GetProcAddress(h_module, PCSTR(func_name.cast()))
                };

                match func_address {
                    Some(addr) => (*first_thunk).u1.Function = addr as u64,
                    None => {
                        let func_name = if image_snap_by_ordinal((*original_first_chunk).u1.Ordinal) {
                            format!("{}", image_ordinal((*original_first_chunk).u1.Ordinal))
                        } else {
                            let import_by_name = address.add((*original_first_chunk).u1.AddressOfData as usize) as *mut IMAGE_IMPORT_BY_NAME;
                            format!("{:?}", CStr::from_ptr(&(*import_by_name).Name as *const i8))
                        };

                        return Err(format!("Failed to find function: {}", func_name));
                    }
                };

                thunk_size += size_of::<IMAGE_THUNK_DATA64>() as isize;
            }
        }
    }

    Ok(())
}

/// Sets the appropriate memory protections for each section of the PE file.
///
/// # Arguments
///
/// * `pe` - A mutable reference to a PE struct representing the loaded PE file.
/// * `address` - The base address of the allocated memory for the PE file.
///
/// # Returns
///
/// A `Result` which is `Ok` if the memory protections are successfully set, or `Err` if there is an error.
fn fixing_memory(pe: &mut PE, address: *mut c_void) -> Result<(), String> {
    unsafe {
        let num_sections = (*pe.nt_header).FileHeader.NumberOfSections;
        let mut section_header = pe.section_header;

        for _ in 0..num_sections {
            if (*section_header).SizeOfRawData == 0 || (*section_header).VirtualAddress == 0 {
                section_header = section_header.add(1);
                continue;
            }

            let characteristics = (*pe.section_header).Characteristics;
            let protection = match (
                characteristics & IMAGE_SCN_MEM_EXECUTE != IMAGE_SECTION_CHARACTERISTICS(0),
                characteristics & IMAGE_SCN_MEM_READ != IMAGE_SECTION_CHARACTERISTICS(0),
                characteristics & IMAGE_SCN_MEM_WRITE != IMAGE_SECTION_CHARACTERISTICS(0),
            ) {
                (true, true, true) => PAGE_EXECUTE_READWRITE,
                (true, true, false) => PAGE_EXECUTE_READ,
                (true, false, true) => PAGE_EXECUTE_WRITECOPY,
                (true, false, false) => PAGE_EXECUTE,
                (false, true, true) => PAGE_READWRITE,
                (false, true, false) => PAGE_READONLY,
                (false, false, true) => PAGE_WRITECOPY,
                _ => PAGE_NOACCESS,
            };

            let mut old_protect = PAGE_PROTECTION_FLAGS(0);
            VirtualProtect(
                address.offset((*pe.section_header).VirtualAddress as isize),
                (*pe.section_header).SizeOfRawData as usize,
                protection,
                &mut old_protect,
            ).map_err(|e| format!("VirtualProtect [{}] Failed With Status: {e}", line!()))?;

            section_header = section_header.add(1);
        }
    }

    Ok(())
}

/// Adjusts the command line arguments for the target binary.
///
/// # Arguments
///
/// * `args` - The command line arguments to be passed to the executable.
fn fixing_arguments(args: &str) -> Result<(), String>  {
    unsafe {
        let peb = get_peb();
        let process_parameters = (*peb).ProcessParameters as *mut RTL_USER_PROCESS_PARAMETERS;
        std::ptr::write_bytes((*process_parameters).CommandLine.Buffer.0, 0, (*process_parameters).CommandLine.Length as usize);

        let current_exe = std::env::current_exe().map_err(|e| format!("Failed to get current exe path: {}", e))?;
        let path_name= format!("\"{}\" {}\0", current_exe.to_string_lossy(), args)
            .encode_utf16()
            .collect::<Vec<u16>>();

        std::ptr::copy_nonoverlapping(path_name.as_ptr(), (*process_parameters).CommandLine.Buffer.0, path_name.len());
        (*process_parameters).CommandLine.Length = (path_name.len() * 2) as u16;
        (*process_parameters).CommandLine.MaximumLength = (path_name.len() * 2) as u16;
    }

    Ok(())
}