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

fn is_debugger_present() {
    unsafe {
        if IsDebuggerPresent().into() {
            println!("[!] Debugger Detected!");
        }
    }
}

fn is_debugger_peb() {
    unsafe {
        let peb = get_peb();
        if (*peb).BeingDebugged == 1 {
            println!("[!] Debugger Detected! [2]");
        }
    }
}

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

    for (_pid, process) in system.processes() {
        for name in &list {
            if process.name() == *name {
                println!("[!] Debugger Detected! [3]");
            }
        }
    }
}

fn breakpoint_hardware() -> Result<(), Error> {
    let mut ctx = CONTEXT {
        ContextFlags: CONTEXT_DEBUG_REGISTERS_AMD64,
        ..Default::default()
    };

    unsafe { GetThreadContext(GetCurrentThread(), &mut ctx) }?;

    if ctx.Dr0 != 0 || ctx.Dr1 != 0 || ctx.Dr2 != 0 || ctx.Dr3 != 0 {
        println!("[!] Debugger Detected! [4]");
    }

    Ok(())
}

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