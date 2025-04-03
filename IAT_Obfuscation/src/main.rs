use std::{
    ffi::{CStr, c_void}, 
    slice::from_raw_parts
};
use ntapi::{
    ntldr::LDR_DATA_TABLE_ENTRY,
    ntpebteb::PEB,
};
use windows::{core::{Error, Result}, Win32::Foundation::E_FAIL};
use windows::Win32::System::{
    Diagnostics::Debug::IMAGE_NT_HEADERS64,
    SystemServices::{
        IMAGE_DOS_HEADER, IMAGE_DOS_SIGNATURE, 
        IMAGE_EXPORT_DIRECTORY, IMAGE_NT_SIGNATURE,
    },
};

fn main() -> Result<()> {
    unsafe {
        // Locate the base address of the DLL manually via PEB traversal
        let address = get_module_address("ntdll.dll")?;

        // Parse PE export table and print all exported function names + addresses
        get_proc_address(address);
    };

    Ok(())
}

/// Manually parses the export table of a module loaded in memory.
///
/// This function emulates the behavior of `GetProcAddress` by navigating the PE structure
/// in memory and printing out all exported symbols, their resolved addresses, and ordinals.
///
/// # Parameters
///
/// * `dll_base` - A pointer to the base of a loaded module (e.g., `ntdll.dll`)
unsafe fn get_proc_address(dll_base: *mut c_void) {
    let module = dll_base as usize;
    let dos_header = module as *mut IMAGE_DOS_HEADER;
    if (*dos_header).e_magic != IMAGE_DOS_SIGNATURE {
        eprintln!("INVALID DOS SIGNATURE");
        return;
    }

    let nt_header = (module + (*dos_header).e_lfanew as usize) as *mut IMAGE_NT_HEADERS64;
    if (*nt_header).Signature != IMAGE_NT_SIGNATURE {
        eprintln!("INVALID NT SIGNATURE");
        return;
    }

    // Locate export directory from DataDirectory[0]
    let export_dir = (module + (*nt_header).OptionalHeader.DataDirectory[0].VirtualAddress as usize) 
        as *const IMAGE_EXPORT_DIRECTORY;

    // Retrieving information from module names
    let names = from_raw_parts((
        module + (*export_dir).AddressOfNames as usize) as *const u32, 
        (*export_dir).NumberOfNames as usize
    );

    // Retrieving information from functions
    let functions = from_raw_parts(
        (module + (*export_dir).AddressOfFunctions as usize) as *const u32, 
        (*export_dir).NumberOfFunctions as usize
    );

    // Retrieving information from ordinals
    let ordinals = from_raw_parts(
        (module + (*export_dir).AddressOfNameOrdinals as usize) as *const u16, 
        (*export_dir).NumberOfNames as usize
    );

    // Iterate over exported names
    for i in 0..(*export_dir).NumberOfNames as usize {
        let name = CStr::from_ptr((module + names[i] as usize) as *const i8)
            .to_str()
            .unwrap_or("");

        let ordinal = ordinals[i] as usize;
        let address = (dll_base as usize + functions[ordinal] as usize) as *mut c_void;

        println!("Name {} | Address: {:x?} | Ordinal: {}", name, address, ordinal);
    }
}

/// Locates the base address of a loaded module using PEB traversal.
///
/// This function walks the `InLoadOrderModuleList` linked list inside the PEB's LDR
/// and matches against the given DLL name (case-insensitive).
///
/// # Parameters
///
/// * `dll` - The lowercase name of the DLL to locate (e.g., `"ntdll.dll"`)
///
/// # Returns
///
/// * `Ok(*mut c_void)` - The base address of the loaded DLL.
/// * `Err(())` - If the DLL is not found in memory.
fn get_module_address(dll: &str) -> Result<*mut c_void> {
    unsafe {
        let peb = NtCurrentPeb();
        let ldr = (*peb).Ldr;
        let mut list_entry = (*ldr).InLoadOrderModuleList.Flink as *mut LDR_DATA_TABLE_ENTRY;
    
        while !(*list_entry).DllBase.is_null() {
            let buffer = from_raw_parts(
                (*list_entry).BaseDllName.Buffer,
                ((*list_entry).BaseDllName.Length / 2) as usize,
            );
    
            let dll_name = String::from_utf16_lossy(&buffer).to_lowercase();
            if dll == dll_name {
                return Ok((*list_entry).DllBase.cast());
            }
    
            list_entry = (*list_entry).InLoadOrderLinks.Flink as *mut LDR_DATA_TABLE_ENTRY;
        }
    }
        
    Err(Error::new(E_FAIL, "Module Not found".into()))
}

/// Retrieves a pointer to the Process Environment Block (PEB) of the current process.
/// 
/// # Returns
/// 
/// * Pointer to the PEB structure.
#[inline(always)]
#[allow(non_snake_case)]
fn NtCurrentPeb() -> *const PEB {
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
unsafe fn __readgsqword(offset: u64) -> u64 {
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
unsafe fn __readfsdword(offset: u32) -> u32 {
    let out: u32;
    core::arch::asm!(
        "mov {:e}, fs:[{:e}]",
        lateout(reg) out,
        in(reg) offset,
        options(nostack, pure, readonly),
    );
    out
}