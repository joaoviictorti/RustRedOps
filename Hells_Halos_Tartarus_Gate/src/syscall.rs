use core::{ffi::c_void, ffi::CStr, ptr::read};
use std::slice::from_raw_parts;
use ntapi::{
    ntldr::LDR_DATA_TABLE_ENTRY,
    ntpebteb::PEB, 
};
use windows_sys::Win32::System::{
    Diagnostics::Debug::IMAGE_NT_HEADERS64, 
    SystemServices::{
        IMAGE_DOS_HEADER, IMAGE_DOS_SIGNATURE, 
        IMAGE_EXPORT_DIRECTORY, IMAGE_NT_SIGNATURE
    }
};

const RANGE: usize = 255;
const UP: isize = -32;
const DOWN: usize = 32;

/// Hashes a string using the DJB2 algorithm.
/// 
/// # Arguments
/// 
/// * `string` - Input string to hash.
/// 
/// # Returns
/// 
/// * 32-bit hash value.
pub fn dbj2(string: &str) -> u32 {
    let mut hash: u32 = 5381;

    for c in string.bytes() {
        hash = ((hash << 5).wrapping_add(hash)).wrapping_add(c as u32);
    }
    hash
}

/// Retrieves the syscall number (SSN) for a given hashed function name.
/// 
/// # Arguments
/// 
/// * `name` - Hashed function name (DJB2 hash).
/// 
/// # Returns
/// 
/// * The syscall number (SSN) as `u16`.
pub fn get_ssn(name: u32) -> u16 {
    let ntdll = get_ntdll_address();
    let export_ntdll = get_export_ntdll(ntdll).expect("[!] Error exporting DLL");
    ssn(name, ntdll, export_ntdll).expect("[!] Error getting syscall number")
}

/// Retrieves the base address of the `ntdll.dll` module.
///
/// This function accesses the Process Environment Block (PEB) and traverses
/// the loader data structures to locate the base address of `ntdll.dll`.
/// 
/// # Returns
///
/// * A pointer to the base address of the `ntdll.dll` module.
pub fn get_ntdll_address() -> *mut c_void {
    unsafe {
        let peb = NtCurrentPeb();
        let ldr_data = ((*(*(*peb).Ldr).InMemoryOrderModuleList.Flink).Flink as *const u8)
            .offset(if cfg!(target_arch = "x86_64") { -0x10 } else { -0x08 }) 
            as *const LDR_DATA_TABLE_ENTRY;
        
        (*ldr_data).DllBase.cast::<c_void>()
    }
}

/// Retrieves the Export Directory of a loaded module (ntdll).
/// 
/// # Arguments
/// 
/// * `dll_base` - Base address of the DLL.
/// 
/// # Returns
/// 
/// * `Ok(IMAGE_EXPORT_DIRECTORY)` if successful.
/// * `Err(String)` if invalid headers are found.
pub fn get_export_ntdll(dll_base: *mut c_void) -> Result<IMAGE_EXPORT_DIRECTORY, String> {
    unsafe {
        let dos_header = dll_base as *const IMAGE_DOS_HEADER;
        if (*dos_header).e_magic != IMAGE_DOS_SIGNATURE {
            return Err(String::from("[!] INVALID DOS SIGNATURE"));
        }
    
        let nt_header = dll_base.wrapping_add((*dos_header).e_lfanew as usize) as *const IMAGE_NT_HEADERS64;
        if (*nt_header).Signature != IMAGE_NT_SIGNATURE {
            return Err(String::from("[!] INVALID NT SIGNATURE"));
        }
    
        let export_directory = dll_base.wrapping_add((*nt_header).OptionalHeader.DataDirectory[0].VirtualAddress as usize) 
            as *const IMAGE_EXPORT_DIRECTORY;
        Ok(read(export_directory))
    }
}

/// Resolves the syscall number (SSN) for a specific function in ntdll, handling multiple gates.
/// 
/// Supports:
/// - Hell's Gate
/// - Halo's Gate
/// - Tartarus Gate
/// 
/// # Arguments
/// 
/// * `function_name` - DJB2 hash of the function name.
/// * `module` - Base address of the module (ntdll).
/// * `export_directory` - Export directory structure of the module.
/// 
/// # Returns
/// 
/// * `Ok(u16)` containing the syscall number.
/// * `Err(())` if not found.
pub fn ssn(
    function_name: u32, 
    module: *mut c_void, 
    export_directory: IMAGE_EXPORT_DIRECTORY
) -> Result<u16, ()> {
    unsafe {
        // Retrieving information from module names
        let names = from_raw_parts(
           (module as usize + export_directory.AddressOfNames as usize) as *const u32, 
            export_directory.NumberOfNames as usize
        );

        // Retrieving information from functions
        let functions = from_raw_parts(
            (module as usize + export_directory.AddressOfFunctions as usize) as *const u32, 
            export_directory.NumberOfFunctions as usize
        );

        // Retrieving information from ordinals
        let ordinals = from_raw_parts(
            (module as usize + export_directory.AddressOfNameOrdinals as usize) as *const u16, 
            export_directory.NumberOfNames as usize
        );
    
        for i in 0..export_directory.NumberOfNames as isize {
            let ordinal = ordinals[i as usize] as usize;
            let address = (module as usize + functions[ordinal] as usize) as *const u8;
            let name = CStr::from_ptr((module as usize + names[i as usize] as usize) as *const i8)
                .to_str()
                .unwrap_or("");
    
            if dbj2(&name) == function_name {
                // Hells Gate
                // MOV R10, RCX
                // MOV RCX, <syscall>
                if read(address) == 0x4C
                    && read(address.add(1)) == 0x8B
                    && read(address.add(2)) == 0xD1
                    && read(address.add(3)) == 0xB8
                    && read(address.add(6)) == 0x00
                    && read(address.add(7)) == 0x00 
                {
                    let high = read(address.add(5)) as u16;
                    let low = read(address.add(4)) as u16;
                    let ssn = (high << 8) | low;
                    return Ok(ssn);
                }
    
                // Halos Gate
                if read(address) == 0xE9 {
                    for idx in 1..RANGE {
                        // check neighboring syscall down
                        if read(address.add(idx * DOWN)) == 0x4C
                            && read(address.add(1 + idx * DOWN)) == 0x8B
                            && read(address.add(2 + idx * DOWN)) == 0xD1
                            && read(address.add(3 + idx * DOWN)) == 0xB8
                            && read(address.add(6 + idx * DOWN)) == 0x00
                            && read(address.add(7 + idx * DOWN)) == 0x00 
                            {
                                let high = read(address.add(5 + idx * DOWN)) as u16;
                                let low = read(address.add(4 + idx * DOWN)) as u16;
                                let ssn = (high << 8) | (low - idx as u16);
                                return Ok(ssn);
                            }
                        // check neighboring syscall up
                        if read(address.offset(idx as isize * UP)) == 0x4c
                            && read(address.offset(1 + idx as isize * UP)) == 0x8B
                            && read(address.offset(2 + idx as isize * UP)) == 0xD1
                            && read(address.offset(3 + idx as isize * UP)) == 0xB8
                            && read(address.offset(6 + idx as isize * UP)) == 0x00
                            && read(address.offset(7 + idx as isize * UP)) == 0x00 
                            {
                                let high = read(address.offset(5 + idx as isize * UP)) as u16;
                                let low = read(address.offset(4 + idx as isize * UP)) as u16;
                                let ssn = (high << 8) | (low + idx as u16);
                                return Ok(ssn);
                            }
                    }
                }
    
                // Tartarus Gate
                if read(address.add(3)) == 0xE9 {
                    for idx in 1..RANGE {
                        // check neighboring syscall down
                        if read(address.add(idx * DOWN)) == 0x4C
                            && read(address.add(1 + idx * DOWN)) == 0x8B
                            && read(address.add(2 + idx * DOWN)) == 0xD1
                            && read(address.add(3 + idx * DOWN)) == 0xB8
                            && read(address.add(6 + idx * DOWN)) == 0x00
                            && read(address.add(7 + idx * DOWN)) == 0x00 
                            {
                                let high = read(address.add(5 + idx * DOWN)) as u16;
                                let low = read(address.add(4 + idx * DOWN)) as u16;
                                let ssn = (high << 8) | (low - idx as u16);
                                return Ok(ssn);
                            }
                        // check neighboring syscall up
                        if read(address.offset(idx as isize * UP)) == 0x4c
                            && read(address.offset(1 + idx as isize * UP)) == 0x8B
                            && read(address.offset(2 + idx as isize * UP)) == 0xD1
                            && read(address.offset(3 + idx as isize * UP)) == 0xB8
                            && read(address.offset(6 + idx as isize * UP)) == 0x00
                            && read(address.offset(7 + idx as isize * UP)) == 0x00 
                            {
                                let high = read(address.offset(5 + idx as isize * UP)) as u16;
                                let low = read(address.offset(4 + idx as isize * UP)) as u16;
                                let ssn = (high << 8) | (low + idx as u16);
                                return Ok(ssn);
                            }
                    }
                }
            }
        }
    }

    Err(())
}

/// Retrieves a pointer to the Process Environment Block (PEB) of the current process.
/// 
/// # Returns
/// 
/// * Pointer to the PEB structure.
#[inline(always)]
#[allow(non_snake_case)]
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