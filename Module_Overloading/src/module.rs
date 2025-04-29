#![allow(non_snake_case)]
#![allow(unused)]

use std::{
    ffi::{c_void, CStr}, 
    mem::transmute, 
    ptr::null_mut, 
    slice::from_raw_parts
};

use windows::{
    core::{Error, Result, PCSTR}, 
    Wdk::{
        Storage::FileSystem::NtCreateSection, 
        System::Memory::{NtMapViewOfSection, ViewShare}
    }, 
    Win32::{
        Foundation::{
            BOOL, E_FAIL, GENERIC_READ, HANDLE, 
            HINSTANCE, STATUS_SUCCESS
        }, 
        Storage::FileSystem::{
            CreateFileA, FILE_ATTRIBUTE_NORMAL, 
            FILE_SHARE_MODE, OPEN_EXISTING
        }
    }
};
use windows::Win32::System::{
    Memory::*,
    Threading::*,
    SystemServices::*,
    Diagnostics::Debug::*,
    WindowsProgramming::IMAGE_THUNK_DATA64,
    LibraryLoader::{GetProcAddress, LoadLibraryA},
};

/// Function type for an executable PE.
pub type Exe = unsafe extern "system" fn() -> BOOL;

/// Function type for a DLL entry (DllMain).
pub type Dll = unsafe extern "system" fn(HINSTANCE, u32, *mut c_void) -> BOOL;

/// Struct representing a loaded module used for module overloading.
#[derive(Debug)]
pub struct Module {
    /// The buffer containing the PE file data.
    pub buffer: Vec<u8>,

    /// The name of the target DLL to overload.
    pub target_dll: String,

    /// Command-line arguments to pass when executing the binary.
    pub args: String,

    /// Pointer to the NT headers of the PE file.
    pub nt_header: *mut IMAGE_NT_HEADERS64,

    /// Pointer to the section headers of the PE file.
    pub section_header: *mut IMAGE_SECTION_HEADER,

    /// Data directory entry for the import table.
    pub import_data: IMAGE_DATA_DIRECTORY,

    /// Data directory entry for the base relocation table.
    pub basereloc: IMAGE_DATA_DIRECTORY,

    /// Indicates whether the PE file is a DLL.
    pub is_dll: bool,
}

impl Module {
    /// Initializes the Module structure by parsing the PE headers from the provided buffer.
    ///
    /// # Arguments
    ///
    /// * `buffer` - The binary content of the PE file.
    /// * `args` - Command-line arguments to be passed (for executables).
    /// * `target_dll` - Path to the target DLL for overloading.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - If the module is parsed correctly.
    /// * `Err(Error)` - If parsing the PE headers fails.
    pub fn new(buffer: Vec<u8>, args: String, target_dll: String) -> Result<Self> {
        unsafe {
            let dos_header = buffer.as_ptr() as *mut IMAGE_DOS_HEADER;
            if (*dos_header).e_magic != IMAGE_DOS_SIGNATURE {
                return Err(Error::new(E_FAIL, "Invalid DOS SIGNATURE"));
            }

            let nt_header =(dos_header as usize + (*dos_header).e_lfanew as usize) as *mut IMAGE_NT_HEADERS64;
            if (*nt_header).Signature != IMAGE_NT_SIGNATURE {
                return Err(Error::new(E_FAIL, "Invalid NT SIGNATURE"));
            }

            let section_header = (nt_header as usize + size_of::<IMAGE_NT_HEADERS64>()) as *mut IMAGE_SECTION_HEADER;
            Ok(Self {
                buffer,
                args,
                target_dll,
                nt_header,
                section_header,
                is_dll: (*nt_header).FileHeader.Characteristics.0 & IMAGE_FILE_DLL.0 != 0,
                import_data: (*nt_header).OptionalHeader.DataDirectory[IMAGE_DIRECTORY_ENTRY_IMPORT.0 as usize],
                basereloc: (*nt_header).OptionalHeader.DataDirectory[IMAGE_DIRECTORY_ENTRY_BASERELOC.0 as usize],
            })
        }
    }

    /// Loads the target DLL into the current process memory using NT native API.
    ///
    /// # Returns
    ///
    /// * `Ok(*mut c_void)` - A pointer to the mapped module.
    /// * `Err(Error)` - If loading the library fails.
    fn load_library(&self) -> Result<*mut c_void> {
        unsafe {
            let dll = std::ffi::CString::new(self.target_dll.clone()).unwrap().into_raw();
            let h_file = CreateFileA(
                PCSTR(dll.cast()),
                GENERIC_READ.0,
                FILE_SHARE_MODE(0),
                None,
                OPEN_EXISTING,
                FILE_ATTRIBUTE_NORMAL,
                None,
            )?;

            let mut section = HANDLE::default();
            let status = NtCreateSection(
                &mut section,
                SECTION_ALL_ACCESS.0,
                None,
                None,
                PAGE_READONLY.0,
                SEC_IMAGE.0,
                Some(h_file),
            );
            
            if status != STATUS_SUCCESS {
                return Err(Error::new(E_FAIL, format!("NtCreateSection Failed With Status: {:?}", status)));
            }

            let mut module = null_mut();
            let mut view_size = 0;
            let status = NtMapViewOfSection(
                section,
                HANDLE(-1isize as *mut c_void),
                &mut module,
                0,
                0,
                None,
                &mut view_size,
                ViewShare,
                0,
                PAGE_EXECUTE_READWRITE.0,
            );

            if status != STATUS_SUCCESS {
                return Err(Error::new(E_FAIL, format!("NtMapViewOfSection Failed With Status: {:?}", status)));
            }

            Ok(module)
        }
    }

    /// Runs the injected module by fixing imports, relocations, and calling the entry point.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If execution is successful.
    /// * `Err(Error)` - If execution fails.
    pub fn run(&self) -> Result<()> {
        let module = self.load_library()?;
        self.prepare(module)?;

        unsafe {
            // Adjusting the arguments (if any)
            self.fixing_arguments()?;

            // Execute Entrypoint
            let entry_point = module.offset((*self.nt_header).OptionalHeader.AddressOfEntryPoint as isize);
            if self.is_dll {
                let DllMain = transmute::<_, Dll>(entry_point);
                DllMain(HINSTANCE(module as *mut c_void), DLL_PROCESS_ATTACH, null_mut());
            } else {
                let Main = transmute::<_, Exe>(entry_point);
                Main();
            }

            Ok(())
        }
    }

    /// Prepares the module by loading sections, fixing IAT, relocations, and memory protections.
    ///
    /// # Arguments
    ///
    /// * `module` - The base address of the mapped module.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If preparation succeeds.
    /// * `Err(Error)` - If any step fails.
    fn prepare(&self, module: *mut c_void) -> Result<()> {
        unsafe {
            // Allocate memory for the image
            let address = VirtualAlloc(
                None,
                (*self.nt_header).OptionalHeader.SizeOfImage as usize,
                MEM_COMMIT | MEM_RESERVE,
                PAGE_READWRITE,
            );

            if address.is_null() {
                return Err(Error::new(E_FAIL, "[!] VirtualAlloc failed to allocate memory"));
            }
        
            // Load sections into memory
            let mut tmp_section = self.section_header;
            for _ in 0..(*self.nt_header).FileHeader.NumberOfSections {
                let dst = (*tmp_section).VirtualAddress as isize;
                let start = (*tmp_section).PointerToRawData as usize;
                let end = start + (*tmp_section).SizeOfRawData as usize;
            
                if end <= self.buffer.len() {
                    let src = &self.buffer[start..end];
                    std::ptr::copy_nonoverlapping(
                        src.as_ptr(),
                        address.offset(dst).cast(),
                        src.len(),
                    );
                } else {
                    return Err(Error::new(E_FAIL, "[!] Section outside the buffer limits"));
                }

                tmp_section = tmp_section.add(1)
            }
        
            // Temporarily set writable protections
            let mut old_protect = PAGE_PROTECTION_FLAGS(0);
            VirtualProtect(
                module, 
                (*self.nt_header).OptionalHeader.SizeOfImage as usize, 
                PAGE_READWRITE, 
                &mut old_protect
            )?;

            // Copy reconstructed PE into the target module memory
            std::ptr::copy_nonoverlapping(address, module, (*self.nt_header).OptionalHeader.SizeOfImage as usize);

            // Adjusting the IAT header
            self.fixing_iat(module)?; 
            
            // Adjusting relocations
            self.realoc_image(module)?;
            
            // Restore headers to read-only
            VirtualProtect(
                module, 
                (*self.nt_header).OptionalHeader.SizeOfHeaders as usize, 
                PAGE_READONLY, 
                &mut old_protect
            )?;

            // Adjusting memory permissions
            self.fixing_memory(module)?;

            Ok(())
        }
    }

    /// Fixes the Import Address Table (IAT) by loading required libraries and resolving function addresses.
    ///
    /// # Arguments
    ///
    /// * `address` - Base address where the PE is loaded in memory.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the IAT was successfully fixed.
    /// * `Err` - If any import could not be resolved.
    fn fixing_iat(&self, address: *mut c_void) -> Result<()> {
        unsafe {
            let import_descriptor = address.offset(self.import_data.VirtualAddress as isize) as *mut IMAGE_IMPORT_DESCRIPTOR;
            let import_descriptors = from_raw_parts(import_descriptor, self.import_data.Size as usize / size_of::<IMAGE_IMPORT_DESCRIPTOR>());
            
            for import in import_descriptors {
                let original_first_chunk_rva = import.Anonymous.OriginalFirstThunk;
                let first_thunk_rva = import.FirstThunk;
                if original_first_chunk_rva == 0 && first_thunk_rva == 0 {
                    break;
                }
            
                let mut thunk_offset = 0;
                let dll_name = address.offset(import.Name as isize) as *const i8;
                let h_module = LoadLibraryA(PCSTR(dll_name.cast())).expect("Error");
                loop {
                    let original_thunk = address.offset(original_first_chunk_rva as isize + thunk_offset) as *const IMAGE_THUNK_DATA64;
                    let first_thunk = address.offset(first_thunk_rva as isize + thunk_offset) as *mut IMAGE_THUNK_DATA64;
                    if (*original_thunk).u1.Function == 0 && (*first_thunk).u1.Function == 0 {
                        break;
                    }

                    // Resolve function by ordinal
                    let adddress = if (*original_thunk).u1.Ordinal & IMAGE_ORDINAL_FLAG64 != 0 {
                        let ordinal = (*original_thunk).u1.Ordinal & 0xffff;
                        GetProcAddress(h_module, PCSTR(ordinal as *const u8))
                    } else {
                        // Resolve function by name
                        let import_by_name = address.add((*original_thunk).u1.AddressOfData as usize) as *const IMAGE_IMPORT_BY_NAME;
                        let name = &(*import_by_name).Name as *const i8;
                        GetProcAddress(h_module, PCSTR(name as *const u8))
                    };

                    match adddress {
                        Some(addr) => (*first_thunk).u1.Function = addr as u64,
                        None => {
                            let func_name = if (*original_thunk).u1.Ordinal & IMAGE_ORDINAL_FLAG64 != 0 {
                                format!("{}", (*original_thunk).u1.Ordinal & 0xffff)
                            } else {
                                let import_by_name = address.add((*original_thunk).u1.AddressOfData as usize) as *mut IMAGE_IMPORT_BY_NAME;
                                format!("{:?}", CStr::from_ptr(&(*import_by_name).Name as *const i8))
                            };
                            
                            return Err(Error::new(E_FAIL, format!("Failed to find function: {}", func_name)))
                        }
                    };

                    thunk_offset += size_of::<IMAGE_THUNK_DATA64>() as isize;
                }
            }
        }

        Ok(())
    }

    /// Adjusts the base relocations of the PE image to match its loaded address.
    ///
    /// # Arguments
    ///
    /// * `address` - Base address where the PE is loaded in memory.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If relocations were successfully applied.
    /// * `Err` - If an unknown relocation type is encountered.
    fn realoc_image(&self, address: *mut c_void) -> Result<()> {
        unsafe {
            let offset = address.wrapping_sub((*self.nt_header).OptionalHeader.ImageBase as usize);
            let mut base_relocation = address.offset(self.basereloc.VirtualAddress as isize) as *mut IMAGE_BASE_RELOCATION;

            while (*base_relocation).VirtualAddress != 0 {
                let mut base_entry = base_relocation.offset(1) as *mut BASE_RELOCATION_ENTRY;
                let block_end = (base_relocation as *mut u8).offset((*base_relocation).SizeOfBlock as isize) as *mut BASE_RELOCATION_ENTRY;

                while base_entry < block_end {
                    let entry_type = (*base_entry).type_();
                    let entry_offset = (*base_entry).offset() as u32;
                    let target_address = address.add(((*base_relocation).VirtualAddress + entry_offset) as usize);

                    match entry_type as u32 {
                        IMAGE_REL_BASED_DIR64 => *(target_address as *mut isize) += offset as isize,
                        IMAGE_REL_BASED_HIGHLOW => *(target_address as *mut u32) = (*(target_address as *mut u32)).wrapping_add(offset as u32),
                        IMAGE_REL_BASED_HIGH => *(target_address as *mut u16) = (*(target_address as *mut u16) as u32).wrapping_add((offset as u32 >> 16) & 0xFFFF) as u16,
                        IMAGE_REL_BASED_LOW => *(target_address as *mut u16) = (*(target_address as *mut u16) as u32).wrapping_add(offset as u32 & 0xFFFF) as u16,
                        IMAGE_REL_BASED_ABSOLUTE => {},
                        _ => return Err(Error::new(E_FAIL, format!("Unknown relocation type: {}", entry_type)))
                    }

                    base_entry = base_entry.add(1);
                }

                base_relocation = base_entry as *mut IMAGE_BASE_RELOCATION;
            }
        }

        Ok(())
    }

    /// Sets appropriate memory protections for each section of the PE.
    ///
    /// # Arguments
    ///
    /// * `address` - Base address where the PE is loaded in memory.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If memory permissions were correctly set.
    /// * `Err` - If VirtualProtect fails.
    fn fixing_memory(&self, address: *mut c_void) -> Result<()> {
        unsafe { 
            let num_sections = (*self.nt_header).FileHeader.NumberOfSections;
            let mut section_header = self.section_header;

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
                )?;

                section_header = section_header.add(1);
            }
        }

        Ok(())
    }

    /// Adjusts the command-line arguments for the PE being executed.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If arguments were successfully modified.
    /// * `Err` - If modification fails.
    fn fixing_arguments(&self) -> Result<()>  {
        unsafe {
            let peb = NtCurrentPeb();
            let process_parameters = (*peb).ProcessParameters as *mut RTL_USER_PROCESS_PARAMETERS;
            std::ptr::write_bytes((*process_parameters).CommandLine.Buffer.0, 0, (*process_parameters).CommandLine.Length as usize);

            let current_exe = std::env::current_exe().unwrap_or("".into());
            let path_name= format!("\"{}\" {}\0", current_exe.to_string_lossy(), self.args)
                .encode_utf16()
                .collect::<Vec<u16>>();

            std::ptr::copy_nonoverlapping(path_name.as_ptr(), (*process_parameters).CommandLine.Buffer.0, path_name.len());
            (*process_parameters).CommandLine.Length = (path_name.len() * 2) as u16;
            (*process_parameters).CommandLine.MaximumLength = (path_name.len() * 2) as u16;
        }

        Ok(())
    }
}

/// Retrieves a pointer to the Process Environment Block (PEB) of the current process.
/// 
/// # Returns
/// 
/// * Pointer to the PEB structure.
#[inline(always)]
pub fn NtCurrentPeb() -> *const PEB {
    unsafe {
        #[cfg(target_arch = "x86_64")]
        return __readgsqword(0x60) as *const PEB;

        #[cfg(target_arch = "x86")]
        return __readfsdword(0x30) as *const PEB;
    }
}

/// Reads a `u64` value from the GS segment at the specified offset.
/// 
/// # Arguments
/// 
/// * `offset` - The offset from the GS base where the value is located.
/// 
/// # Returns
/// 
/// * The value read from the GS segment.
#[inline(always)]
#[cfg(target_arch = "x86_64")]
pub unsafe fn __readgsqword(offset: u64) -> u64 {
    let out: u64;
    core::arch::asm!(
        "mov {}, gs:[{:e}]",
        lateout(reg) out,
        in(reg) offset,
        options(nostack, pure, readonly),
    );
    out
}

/// Reads a `u32` value from the FS segment at the specified offset.
/// 
/// # Arguments
/// 
/// * `offset` - The offset from the FS base where the value is located.
/// 
/// # Returns
/// 
/// * The value read from the FS segment.
#[inline(always)]
#[cfg(target_arch = "x86")]
pub unsafe fn __readfsdword(offset: u32) -> u32 {
    let out: u32;
    core::arch::asm!(
        "mov {:e}, fs:[{:e}]",
        lateout(reg) out,
        in(reg) offset,
        options(nostack, pure, readonly),
    );
    out
}

/// Struct representing a base relocation entry.
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct BASE_RELOCATION_ENTRY {
    pub data: u16,
}

impl BASE_RELOCATION_ENTRY {
    pub fn offset(&self) -> u16 {
        self.data & 0x0FFF
    }

    pub fn type_(&self) -> u16 {
        (self.data >> 12) & 0xF
    }
}
