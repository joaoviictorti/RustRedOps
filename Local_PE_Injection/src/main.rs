#![allow(unused_assignments)]

mod utils;
mod cmd;

use std::{ffi::c_void, mem::{size_of, transmute}, path::Path, ptr::null_mut};
use clap::Parser;
use utils::{image_ordinal, image_snap_by_ordinal, DllMain, Main, BASE_RELOCATION_ENTRY, get_peb};
use cmd::Args;
use windows::{
    core::PCSTR,
    Win32::{Foundation::{FARPROC, HINSTANCE}, 
    System::{
        Diagnostics::Debug::*, 
        LibraryLoader::{GetProcAddress, LoadLibraryA}, 
        Memory::*, 
        SystemServices::*, 
        Threading::*, 
        WindowsProgramming::IMAGE_THUNK_DATA64
    }},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let pe = &args.pe;
    let path = Path::new(&pe);
    let mut param = String::from("");
    let mut export = String::from("");

    let extension = path.extension().and_then(|ext| ext.to_str());
    match extension {
        Some("exe") => {
            if let Some(arg) = &args.arg {
                param = arg.to_string()
            }
        },
        Some("dll") => {
            if let Some(function) = &args.export {
                export = function.to_string()
            }
        },
        _ => {
            panic!("The file provided does not have a valid .exe or .dll extension");
        },
    }

    let buffer = std::fs::read(&args.pe)?;
    let mut pe = PE::new(buffer).unwrap(); 
    pe.local_pe_exec(param, export)?;

    Ok(())
}

#[derive(Debug)]
pub struct PE {
    pub file_buffer: Vec<u8>,
    pub nt_header: *mut IMAGE_NT_HEADERS64,
    pub section_header: *mut IMAGE_SECTION_HEADER,
    pub entry_import_data: IMAGE_DATA_DIRECTORY,
    pub entry_basereloc_data: IMAGE_DATA_DIRECTORY,
    pub entry_tls_data: IMAGE_DATA_DIRECTORY,
    pub entry_exception: IMAGE_DATA_DIRECTORY,
    pub entry_export_data: IMAGE_DATA_DIRECTORY,
    pub is_dll: bool,
}

impl PE {
    fn new(buffer: Vec<u8>) -> Option<Self> {
        unsafe {
            let dos_header = buffer.as_ptr() as *mut IMAGE_DOS_HEADER;
            if (*dos_header).e_magic != IMAGE_DOS_SIGNATURE {
                return None;
            }
    
            let nt_header = (dos_header as usize + (*dos_header).e_lfanew as usize) as *mut IMAGE_NT_HEADERS64;
            if (*nt_header).Signature != IMAGE_NT_SIGNATURE {
                return None;
            }
    
            let section_header = (nt_header as usize + size_of::<IMAGE_NT_HEADERS64>()) as *mut IMAGE_SECTION_HEADER;
    
            Some(Self {
                file_buffer: buffer,
                nt_header,
                section_header,
                entry_import_data: (*nt_header).OptionalHeader.DataDirectory[IMAGE_DIRECTORY_ENTRY_IMPORT.0 as usize],
                entry_basereloc_data: (*nt_header).OptionalHeader.DataDirectory[IMAGE_DIRECTORY_ENTRY_BASERELOC.0 as usize],
                entry_tls_data: (*nt_header).OptionalHeader.DataDirectory[IMAGE_DIRECTORY_ENTRY_TLS.0 as usize],
                entry_exception: (*nt_header).OptionalHeader.DataDirectory[IMAGE_DIRECTORY_ENTRY_EXCEPTION.0 as usize],
                entry_export_data: (*nt_header).OptionalHeader.DataDirectory[IMAGE_DIRECTORY_ENTRY_EXPORT.0 as usize],
                is_dll: (*nt_header).FileHeader.Characteristics.0 & IMAGE_FILE_DLL.0 != 0,
            })
        }
    }

    fn local_pe_exec(&mut self, param: String, export: String) -> Result<(), String> {
        let address = unsafe {
            VirtualAlloc(
                None,
                (*self.nt_header).OptionalHeader.SizeOfImage as usize,
                MEM_COMMIT | MEM_RESERVE,
                PAGE_READWRITE,
            )
        };
    
        let mut tmp_section = self.section_header;
        let mut export_address = null_mut();
    
        unsafe {
            for _ in 0..(*self.nt_header).FileHeader.NumberOfSections {
                let dst = (*tmp_section).VirtualAddress as isize;
                let src_start = (*tmp_section).PointerToRawData as usize;
                let src_end = src_start + (*tmp_section).SizeOfRawData as usize;
            
                if src_end <= self.file_buffer.len() {
                    let src = &self.file_buffer[src_start..src_end];
                    std::ptr::copy_nonoverlapping(
                        src.as_ptr(),
                        address.offset(dst) as _,
                        src.len(),
                    );
    
                } else {
                    return Err("[!] Section outside the buffer limits".to_string());
                }
    
                tmp_section = (tmp_section as usize + size_of::<IMAGE_SECTION_HEADER>()) as *mut IMAGE_SECTION_HEADER;
            }
        }
    
        self.resolve_import(address)?; 
        self.realoc_image(address)?;
        self.resolve_memory(address)?;
    
        if self.entry_export_data.Size != 0 && self.entry_export_data.VirtualAddress != 0 && !export.is_empty()  {
            export_address = self.export_function_address(address as usize, &export).expect("[!] Export Function Address");
        }
    
        if self.entry_exception.Size != 0 {
            let func_entries = unsafe { std::slice::from_raw_parts(
                address.offset(self.entry_exception.VirtualAddress as isize) as *mut IMAGE_RUNTIME_FUNCTION_ENTRY,
                (self.entry_exception.Size / size_of::<IMAGE_RUNTIME_FUNCTION_ENTRY>() as u32) as usize,
            ) };
            let status = unsafe { RtlAddFunctionTable(func_entries, address as u64) };
    
            if !status.as_bool() {
                return Err("[!] Failed to call RtlAddFunctionTable".to_string());
            }   
        }
    
        if self.entry_tls_data.Size != 0 {
            unsafe { 
                let img_tls_directory = address.offset(self.entry_tls_data.VirtualAddress as isize) as *mut IMAGE_TLS_DIRECTORY64;
                let img_tls_callback = (*img_tls_directory).AddressOfCallBacks as *mut PIMAGE_TLS_CALLBACK;
    
                let mut i = 0;
                while let Some(callback) = *img_tls_callback.offset(i) {
                    callback(address, DLL_PROCESS_ATTACH, null_mut());
                    i += 1;
                } 
            }   
        }
    
        self.fixing_arguments(param);
        
        unsafe {
            if self.is_dll {
                let entry_point = address.offset((*self.nt_header).OptionalHeader.AddressOfEntryPoint as isize); 
                let func_dll = transmute::<_, DllMain>(entry_point);
                func_dll(HINSTANCE(address as isize), DLL_PROCESS_ATTACH, null_mut());
    
                if !export_address.is_null() {
                    let htread = CreateThread(
                        None, 
                        0, 
                        std::mem::transmute(export_address), 
                        None, 
                        THREAD_CREATION_FLAGS(0), 
                        None
                    ).expect("[!] Error when calling CreateThread");
                    
                    WaitForSingleObject(htread, INFINITE);
                }
                
            } else {
                let entry_point = address.offset((*self.nt_header).OptionalHeader.AddressOfEntryPoint as isize);
                let func = transmute::<_, Main>(entry_point);
                func();
            }

            Ok(())
        }
    }

    fn realoc_image(&self, address: *mut c_void) -> Result<(), String> {
        unsafe {
            let mut base_relocation = address.offset(self.entry_basereloc_data.VirtualAddress as isize) as *mut IMAGE_BASE_RELOCATION;
            let offset = address.wrapping_sub((*self.nt_header).OptionalHeader.ImageBase as usize);
            
            while (*base_relocation).VirtualAddress != 0 {
                let mut base_entry = base_relocation.offset(1) as *mut BASE_RELOCATION_ENTRY;
                let block_end = (base_relocation as *mut u8).offset((*base_relocation).SizeOfBlock as isize) as *mut BASE_RELOCATION_ENTRY;
                
                while base_entry < block_end {
                    let entry = *base_entry;
                    let entry_type = entry.type_();
                    let entry_offset = entry.offset() as u32;
                    let target_address = address.add(((*base_relocation).VirtualAddress + entry_offset) as usize);
        
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
                            return Err("[!] Unknown relocation type".to_string());
                        }
                    }
        
                    base_entry = base_entry.offset(1);
                }
        
                base_relocation = base_entry as *mut IMAGE_BASE_RELOCATION;
            }    
        }

        Ok(())
    }

    fn resolve_import(&mut self, address: *mut c_void) -> Result<(), String> {
        unsafe {
            // Calculate the number of entries in the import table
            let entries = (self.entry_import_data.Size as usize / size_of::<IMAGE_IMPORT_DESCRIPTOR>()) as u32;
            let img_import_descriptor = address.offset(self.entry_import_data.VirtualAddress as isize) as *mut IMAGE_IMPORT_DESCRIPTOR;

            for i in 0..entries {
                let img_import_descriptor = img_import_descriptor.offset(i as isize);
                let original_first_chunk_rva = (*img_import_descriptor).Anonymous.OriginalFirstThunk;
                let first_thunk_rva = (*img_import_descriptor).FirstThunk;

                // Break if both RVAs are zero
                if original_first_chunk_rva == 0 && first_thunk_rva == 0 {
                    break;
                }

                // Retrieve the DLL name
                let dll_name = address.offset((*img_import_descriptor).Name as isize) as *const i8;
                let h_module = LoadLibraryA(PCSTR(dll_name as _)).expect("[!] Error loading library");

                // Initialize thunk size
                let mut thunk_size = 0;

                loop {
                    let original_first_chunk = address.offset(original_first_chunk_rva as isize + thunk_size) as *mut IMAGE_THUNK_DATA64;
                    let first_thunk = address.offset(first_thunk_rva as isize + thunk_size) as *mut IMAGE_THUNK_DATA64;
                    let mut func_address: FARPROC = Default::default();
                    
                    // Break if both function pointers are zero
                    if (*original_first_chunk).u1.Function == 0 && (*first_thunk).u1.Function == 0  {
                        break;
                    }

                    // Check if the function is by ordinal or by name
                    if image_snap_by_ordinal((*original_first_chunk).u1.Ordinal) {
                        let ordinal = image_ordinal((*original_first_chunk).u1.Ordinal);
                        func_address = GetProcAddress(h_module, PCSTR(ordinal as _));
                    } else {
                        let image_import_name = address.offset((*original_first_chunk).u1.AddressOfData as isize) as *mut IMAGE_IMPORT_BY_NAME;
                        let name = &(*image_import_name).Name as *const i8;
                        func_address = GetProcAddress(h_module, PCSTR(name as _));
                    }

                    match func_address {
                        Some(f) => {
                            (*first_thunk).u1.Function = f as *const () as u64;
                        },
                        None => {
                            return Err("[!] The expected function was not found".to_string());
                        }
                    }

                    // Increment the thunk size
                    thunk_size += size_of::<IMAGE_THUNK_DATA64>() as isize;
                }
            }
        }

        Ok(())
    }

    fn resolve_memory(&mut self, address: *mut c_void) -> Result<(), String> {
        unsafe { 
            for _ in 0..(*self.nt_header).FileHeader.NumberOfSections {
                let mut protection = PAGE_PROTECTION_FLAGS(0);
                let image_section_characteristics = IMAGE_SECTION_CHARACTERISTICS(0);
                if (*self.section_header).SizeOfRawData == 0 || (*self.section_header).VirtualAddress == 0 {
                    continue;
                } 

                if (*self.section_header).Characteristics & IMAGE_SCN_MEM_WRITE != image_section_characteristics {
                    protection = PAGE_WRITECOPY
                }

                if (*self.section_header).Characteristics & IMAGE_SCN_MEM_READ != image_section_characteristics {
                    protection = PAGE_READONLY
                }

                if (*self.section_header).Characteristics & IMAGE_SCN_MEM_WRITE != image_section_characteristics  
                    && (*self.section_header).Characteristics & IMAGE_SCN_MEM_READ != image_section_characteristics {
                    protection = PAGE_READWRITE
                }

                if (*self.section_header).Characteristics & IMAGE_SCN_MEM_EXECUTE != image_section_characteristics {
                    protection = PAGE_EXECUTE
                }

                if (*self.section_header).Characteristics & IMAGE_SCN_MEM_EXECUTE != image_section_characteristics
                    && (*self.section_header).Characteristics & IMAGE_SCN_MEM_WRITE != image_section_characteristics {
                    protection = PAGE_EXECUTE_WRITECOPY
                }

                if (*self.section_header).Characteristics & IMAGE_SCN_MEM_EXECUTE != image_section_characteristics 
                    && (*self.section_header).Characteristics & IMAGE_SCN_MEM_READ != image_section_characteristics {
                        protection = PAGE_EXECUTE_READ
                }

                if (*self.section_header).Characteristics & IMAGE_SCN_MEM_EXECUTE != image_section_characteristics 
                    && (*self.section_header).Characteristics & IMAGE_SCN_MEM_WRITE != image_section_characteristics
                    && (*self.section_header).Characteristics & IMAGE_SCN_MEM_READ != image_section_characteristics {
                        protection = PAGE_EXECUTE_READWRITE;
                }

                let mut old_protect = PAGE_PROTECTION_FLAGS(0);
                VirtualProtect(
                    address.offset((*self.section_header).VirtualAddress as isize),
                    (*self.section_header).SizeOfRawData as usize, 
                    protection, 
                    &mut old_protect,
                ).expect("Error when calling VirtualProtect");

                self.section_header = (self.section_header as usize + size_of::<IMAGE_SECTION_HEADER>()) as *mut IMAGE_SECTION_HEADER;
            }
        }

        Ok(())
    }

    fn export_function_address(&self, address: usize, target_name: &str) -> Result<*mut c_void, String> {
        unsafe {
            let img_export_directory = (address + self.entry_export_data.VirtualAddress as usize) as *mut IMAGE_EXPORT_DIRECTORY;
            let names = (address + (*img_export_directory).AddressOfNames as usize) as *const u32;
            let addresses = (address + (*img_export_directory).AddressOfFunctions as usize) as *const u32;
            let ordinals = (address + (*img_export_directory).AddressOfNameOrdinals as usize) as *const u16;

            for i in 0..(*img_export_directory).NumberOfFunctions as isize {
                let name_offset = *names.offset(i) as usize;
                let function_name = std::ffi::CStr::from_ptr((address + name_offset) as *const i8).to_string_lossy();
                let function_rva = *addresses.offset(*ordinals.offset(i) as isize) as usize;
                let function_address = (address + function_rva) as *mut c_void;

                if function_name == target_name {
                    return Ok(function_address);
                }           
            }

            Err("Function address not found".to_string())
        }
    }

    fn fixing_arguments(&self, args: String) {
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
}
