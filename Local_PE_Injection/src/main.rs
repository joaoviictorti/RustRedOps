#![allow(unused_assignments)]

mod utils;
mod cmd;

use {
    cmd::Args,
    clap::Parser,
    std::{
        ptr::null_mut, ffi::{c_void, CStr},
        mem::{size_of, transmute},
    },
    utils::{
        image_ordinal, image_snap_by_ordinal, get_peb,
        Dll, Exe, BASE_RELOCATION_ENTRY, PE,
    },   
};

use windows::{
    core::PCSTR,
    Win32::{
        Foundation::{GetLastError, HINSTANCE}, 
        System::{
            Memory::*, 
            Threading::*,
            SystemServices::*,
            Diagnostics::Debug::*, 
            WindowsProgramming::IMAGE_THUNK_DATA64,
            LibraryLoader::{GetProcAddress, LoadLibraryA}, 
        }
    },
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let path = std::path::Path::new(&args.pe);

    let (param, export) = match path.extension().and_then(|ext| ext.to_str()) {
        Some("exe") => (args.arg.clone().unwrap_or_default(), String::new()),
        Some("dll") => (String::new(), args.export.clone().unwrap_or_default()),
        _ => { 
            eprintln!("The supplied file does not have a valid extension (.exe or .dll)");
            return Ok(())
        }
    };

    let buffer = std::fs::read(&args.pe).map_err(|e| format!("Error reading PE file: {e}"))?;
    let mut pe = initialize_pe(buffer)?;

    // Load the executable or DLL
    load_exe(&mut pe, param, export)?;

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
/// A `Result` containing a `PE` struct if the initialization is successful, or a `String` error message if it fails.
fn initialize_pe(buffer: Vec<u8>) -> Result<PE, String> {
    unsafe {
        let dos_header = buffer.as_ptr() as *mut IMAGE_DOS_HEADER;
        if (*dos_header).e_magic != IMAGE_DOS_SIGNATURE {
            return Err(String::from("Invalid DOS SIGNATURE"));
        }

        let nt_header = (dos_header as usize + (*dos_header).e_lfanew as usize) as *mut IMAGE_NT_HEADERS64;
        if (*nt_header).Signature != IMAGE_NT_SIGNATURE {
            return Err(String::from("INVALID NT SIGNATURE"));
        }

        let section_header = (nt_header as usize + size_of::<IMAGE_NT_HEADERS64>()) as *mut IMAGE_SECTION_HEADER;
        let pe = PE {
            buffer,
            nt_header,
            section_header,
            is_dll: (*nt_header).FileHeader.Characteristics.0 & IMAGE_FILE_DLL.0 != 0,
            entry_tls_data: (*nt_header).OptionalHeader.DataDirectory[IMAGE_DIRECTORY_ENTRY_TLS.0 as usize],
            entry_import_data: (*nt_header).OptionalHeader.DataDirectory[IMAGE_DIRECTORY_ENTRY_IMPORT.0 as usize],
            entry_export_data: (*nt_header).OptionalHeader.DataDirectory[IMAGE_DIRECTORY_ENTRY_EXPORT.0 as usize],
            entry_exception: (*nt_header).OptionalHeader.DataDirectory[IMAGE_DIRECTORY_ENTRY_EXCEPTION.0 as usize],
            entry_basereloc_data: (*nt_header).OptionalHeader.DataDirectory[IMAGE_DIRECTORY_ENTRY_BASERELOC.0 as usize],
        };

        Ok(pe)
    }
}

fn load_exe(pe: &mut PE, param: String, export: String) -> Result<(), String> {
    unsafe {
        // Allocate memory for the image
        let address = VirtualAlloc(
            None,
            (*pe.nt_header).OptionalHeader.SizeOfImage as usize,
            MEM_COMMIT | MEM_RESERVE,
            PAGE_READWRITE,
        );

        if address.is_null() {
            return Err("[!] VirtualAlloc failed to allocate memory".to_string());
        }
    
        // Load sections into memory
        let mut tmp_section = pe.section_header;
        for _ in 0..(*pe.nt_header).FileHeader.NumberOfSections {
            let dst = (*tmp_section).VirtualAddress as isize;
            let src_start = (*tmp_section).PointerToRawData as usize;
            let src_end = src_start + (*tmp_section).SizeOfRawData as usize;
        
            if src_end <= pe.buffer.len() {
                let src = &pe.buffer[src_start..src_end];
                std::ptr::copy_nonoverlapping(
                    src.as_ptr(),
                    address.offset(dst) as _,
                    src.len(),
                );
            } else {
                return Err("[!] Section outside the buffer limits".to_string());
            }

            tmp_section = tmp_section.add(1)
        }
    
        // Adjusting the IAT header
        fixing_iat(pe, address)?; 
        
        // Adjusting relocations
        realoc_image(pe, address)?;
        
        // Adjusting memory permissions
        fixing_memory(pe, address)?;
    
        // Resolve exports if applicable
        let mut export_address = null_mut();
        if pe.entry_export_data.Size != 0 
            && pe.entry_export_data.VirtualAddress != 0 
            && !export.is_empty()  
        {
            export_address = export_function_address(pe, address as usize, &export)?;
        }
    
        // Add Exception Table
        if pe.entry_exception.Size != 0 {
            let func_entries = std::slice::from_raw_parts(
                address.offset(pe.entry_exception.VirtualAddress as isize) as *mut IMAGE_RUNTIME_FUNCTION_ENTRY,
                (pe.entry_exception.Size / size_of::<IMAGE_RUNTIME_FUNCTION_ENTRY>() as u32) as usize,
            );

            if !RtlAddFunctionTable(func_entries, address as u64).as_bool() {
                return Err(format!("[!] RtlAddFunctionTable Failed With Error: {:?}", GetLastError()));
            }   
        }
    
        // Execute entry point
        if pe.entry_tls_data.Size != 0 {
            let img_tls_directory = address.offset(pe.entry_tls_data.VirtualAddress as isize) as *mut IMAGE_TLS_DIRECTORY64;
            let img_tls_callback = (*img_tls_directory).AddressOfCallBacks as *mut PIMAGE_TLS_CALLBACK;

            let mut i = 0;
            while let Some(callback) = *img_tls_callback.offset(i) {
                callback(address, DLL_PROCESS_ATTACH, null_mut());
                i += 1;
            } 
        }
    
        // Adjusting the arguments (if any)
        fixing_arguments(&param)?;
        
        let entry_point = address.offset((*pe.nt_header).OptionalHeader.AddressOfEntryPoint as isize);
        if pe.is_dll {
            let func_dll = transmute::<_, Dll>(entry_point);
            func_dll(HINSTANCE(address as isize), DLL_PROCESS_ATTACH, null_mut());

            if !export_address.is_null() {
                let h_thread = CreateThread(
                    None, 
                    0, 
                    transmute(export_address), 
                    None, 
                    THREAD_CREATION_FLAGS(0), 
                    None
                ).map_err(|e| format!("[!] CreateThread Failed With Error: {e}"))?;
                
                WaitForSingleObject(h_thread, INFINITE);
            }
        } else {
            let func_exe = transmute::<_, Exe>(entry_point);
            func_exe();
        }

        Ok(())
    }
}

/// Fixes the Import Address Table (IAT) by loading the required libraries and resolving function addresses.
///
/// # Arguments
///
/// * `pe` - A reference to the `PE` structure representing the loaded PE file.
/// * `address` - The base address where the PE file is loaded in memory.
///
/// # Returns
///
/// - `Ok(())` if all imports are resolved and the IAT is successfully fixed.
/// - `Err(String)` containing an error message if:
///     - A required library cannot be loaded.
///     - A function cannot be resolved either by name or ordinal.
fn fixing_iat(pe: &PE, address: *mut c_void) -> Result<(), String> {
    unsafe {
        let img_import_descriptor = address.offset(pe.entry_import_data.VirtualAddress as isize) as *mut IMAGE_IMPORT_DESCRIPTOR;
        let import_descriptors = std::slice::from_raw_parts(
            img_import_descriptor, 
            pe.entry_import_data.Size as usize / size_of::<IMAGE_IMPORT_DESCRIPTOR>()
        );
        
        for descriptor in import_descriptors {
            let original_first_chunk_rva = descriptor.Anonymous.OriginalFirstThunk;
            let first_thunk_rva = descriptor.FirstThunk;
            if original_first_chunk_rva == 0 && first_thunk_rva == 0 {
                break;
            }
        
            let mut thunk_offset = 0;
            let dll_name = address.offset(descriptor.Name as isize) as *const i8;
            let h_module = LoadLibraryA(PCSTR(dll_name as _))
                .map_err(|e| format!("LoadLibrary Failed With Status: {e}"))?;

            loop {
                let original_thunk = address.offset(original_first_chunk_rva as isize + thunk_offset) as *const IMAGE_THUNK_DATA64;
                let first_thunk = address.offset(first_thunk_rva as isize + thunk_offset) as *mut IMAGE_THUNK_DATA64;
                if (*original_thunk).u1.Function == 0 && (*first_thunk).u1.Function == 0 {
                    break;
                }

                let func_address = if image_snap_by_ordinal((*original_thunk).u1.Ordinal) {
                    // Resolve function by ordinal
                    let ordinal = image_ordinal((*original_thunk).u1.Ordinal);
                    GetProcAddress(h_module, PCSTR(ordinal as *const u8))
                } else {
                    // Resolve function by name
                    let import_by_name = address.add((*original_thunk).u1.AddressOfData as usize) as *const IMAGE_IMPORT_BY_NAME;
                    let func_name = &(*import_by_name).Name as *const i8;
                    GetProcAddress(h_module, PCSTR(func_name as *const u8))
                };

                match func_address {
                    Some(addr) => (*first_thunk).u1.Function = addr as *const () as u64,
                    None => {
                        let func_name = if image_snap_by_ordinal((*original_thunk).u1.Ordinal) {
                            format!("{}", image_ordinal((*original_thunk).u1.Ordinal))
                        } else {
                            let import_by_name = address.add((*original_thunk).u1.AddressOfData as usize) as *mut IMAGE_IMPORT_BY_NAME;
                            format!("{:?}", CStr::from_ptr(&(*import_by_name).Name as *const i8))
                        };
                        return Err(format!("Failed to find function: {}", func_name));
                    }
                };

                thunk_offset += size_of::<IMAGE_THUNK_DATA64>() as isize;
            }
        }
    }

    Ok(())
}

/// Adjusts the base relocations of the PE file to the new address.
///
/// # Arguments
///
/// * `pe` - A mutable reference to the `PE` structure representing the loaded PE file.
/// * `address` - The base address where the PE file is loaded in memory.
///
/// # Returns
///
/// - `Ok(())` if all base relocations are successfully adjusted.
/// - `Err(String)` containing an error message if an unknown relocation type is encountered.
fn realoc_image(pe: &mut PE, address: *mut c_void) -> Result<(), String> {
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

/// Sets the appropriate memory protections for each section of the PE file.
///
/// # Arguments
///
/// * `pe` - A mutable reference to the `PE` structure representing the loaded PE file.
/// * `address` - The base address where the PE file is loaded in memory.
///
/// # Returns
///
/// - `Ok(())` if all memory protections are successfully set for each section.
/// - `Err(String)` containing an error message if setting memory permissions fails.
fn fixing_memory(pe: &mut PE, address: *mut c_void) -> Result<(), String> {
    unsafe { 
        let num_sections = (*pe.nt_header).FileHeader.NumberOfSections;
        let mut section_header = pe.section_header;

        for _ in 0..num_sections {
            if (*section_header).SizeOfRawData == 0 || (*section_header).VirtualAddress == 0 {
                section_header = section_header.add(1);
                continue;
            }

            let characteristics = (*section_header).Characteristics;
            let protection = match (
                (characteristics & IMAGE_SCN_MEM_EXECUTE) != IMAGE_SECTION_CHARACTERISTICS(0),
                (characteristics & IMAGE_SCN_MEM_READ) != IMAGE_SECTION_CHARACTERISTICS(0),
                (characteristics & IMAGE_SCN_MEM_WRITE) != IMAGE_SECTION_CHARACTERISTICS(0),
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
                address.offset((*section_header).VirtualAddress as isize),
                (*section_header).SizeOfRawData as usize,
                protection,
                &mut old_protect,
            ).map_err(|e| format!("VirtualProtect [{}] Failed With Status: {e}", line!()))?;

            section_header = section_header.add(1);
        }
    }

    Ok(())
}

/// Adjusts the command-line arguments for the target binary.
///
/// # Arguments
///
/// * `args` - A string containing the command-line arguments to pass to the executable.
///
/// # Returns
///
/// - `Ok(())` if the arguments are successfully adjusted in the process's PEB.
/// - `Err(String)` containing an error message if:
///     - The current executable path cannot be retrieved.
///     - The command-line buffer cannot be updated properly.
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

/// Resolves the address of a specific export function within the PE file.
///
/// # Arguments
///
/// * `pe` - A mutable reference to the `PE` structure representing the loaded PE file.
/// * `address` - The base address where the PE file is loaded in memory.
/// * `target_name` - The name of the export function to locate.
/// 
/// # Returns
/// 
/// - `Ok(*mut c_void)` containing a pointer to the resolved function address if successful.
/// - `Err(String)` containing an error message if the function cannot be found.
fn export_function_address(pe: &mut PE, address: usize, target_name: &str) -> Result<*mut c_void, String> {
    unsafe {
        let img_export_directory = (address + pe.entry_export_data.VirtualAddress as usize) as *mut IMAGE_EXPORT_DIRECTORY;
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
