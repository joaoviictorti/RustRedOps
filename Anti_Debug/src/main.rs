use sysinfo::System;
use windows::core::Error;
use windows::Win32::System::{
    Kernel::NT_TIB,
    Threading::{GetCurrentThread, PEB, TEB},
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
        let peb = get_peb();
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

/// Retrieves the `PEB` pointer for the current process.
unsafe fn get_peb() -> *mut PEB {
    let teb_offset = ntapi::FIELD_OFFSET!(NT_TIB, Self_) as u32;

    #[cfg(target_arch = "x86_64")]
    {
        use ntapi::winapi_local::um::winnt::__readgsqword;

        let teb = __readgsqword(teb_offset) as *mut TEB;
        return (*teb).ProcessEnvironmentBlock;
    }

    #[cfg(target_arch = "x86")]
    {
        use ntapi::winapi_local::um::winnt::__readfsdword;
        let teb = __readfsdword(teb_offset) as *mut TEB;
        return (*teb).ProcessEnvironmentBlock;
    }
}