use sysinfo::System;
use windows::core::Error;
use windows::Win32::System::{
    Threading::{GetCurrentThread, PEB},
    Diagnostics::Debug::{
        GetThreadContext, IsDebuggerPresent, 
        CONTEXT, CONTEXT_DEBUG_REGISTERS_AMD64
    }
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    is_debugger_present();
    is_debugger_peb();
    process_list();
    breakpoint_hardware()?;

    Ok(())
}

/// Checks if a debugger is attached using the `IsDebuggerPresent` Windows API.
///
/// If a debugger is detected, it logs a message to the console.
fn is_debugger_present() {
    unsafe {
        if IsDebuggerPresent().into() {
            println!("[!] Debugger Detected! [{}]", line!());
        }
    }
}

/// Checks if the `BeingDebugged` flag in the `PEB` is set.
///
/// This function uses the `get_peb` function to access the `PEB` and determines 
/// if the process is being debugged.
///
/// If the flag is set, it logs a message to the console.
fn is_debugger_peb() {
    unsafe {
        let peb = NtCurrentPeb();
        if (*peb).BeingDebugged == 1 {
            println!("[!] Debugger Detected! [{}]", line!());
        }
    }
}

/// Scans the system for processes matching known debugger names.
///
/// It uses the `sysinfo` crate to retrieve all running processes and compares their names
/// against a predefined list of known debugger executables.
///
/// If a match is found, it logs a message to the console.
fn process_list() {
    let list = vec![
        "x64dbg.exe",
        "ida.exe",
        "ida64.exe",
        "VsDebugConsole.exe",
        "msvsmon.exe",
        "x32dbg.exe"
    ];

    let mut system = System::new_all();

    system.refresh_all();
    for (_, process) in system.processes() {
        for name in &list {
            if process.name() == *name {
                println!("[!] Debugger Detected! [{}]", line!());
            }
        }
    }
}

/// Checks hardware debug registers (`Dr0`, `Dr1`, `Dr2`, `Dr3`) for breakpoints.
///
/// This function retrieves the current thread's context using `GetThreadContext`
/// and inspects the debug registers to detect any hardware breakpoints.
///
/// If any debug register is non-zero, it logs a message to the console.
///
/// # Errors
/// 
/// Returns an error if `GetThreadContext` fails.
fn breakpoint_hardware() -> Result<(), Error> {
    let mut ctx = CONTEXT {
        ContextFlags: CONTEXT_DEBUG_REGISTERS_AMD64,
        ..Default::default()
    };

    unsafe { GetThreadContext(GetCurrentThread(), &mut ctx) }?;

    if ctx.Dr0 != 0 || ctx.Dr1 != 0 || ctx.Dr2 != 0 || ctx.Dr3 != 0 {
        println!("[!] Debugger Detected! [{}]", line!());
    }

    Ok(())
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
