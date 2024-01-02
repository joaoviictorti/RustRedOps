use std::arch::asm;
use sysinfo::System;
use windows::Win32::System::Diagnostics::Debug::{
    GetThreadContext, IsDebuggerPresent, CONTEXT, CONTEXT_DEBUG_REGISTERS_AMD64
};
use windows::Win32::System::Threading::{GetCurrentThread, PEB};

fn main() {
    is_debugger_present();
    is_debugger_peb();
    process_list();
    breakpoint_hardware();
}

fn is_debugger_present() {
    unsafe {
        if IsDebuggerPresent().into() {
            println!("[!] Debugger Detected!");
        }
    }
}

fn is_debugger_peb() {
    #[cfg(target_arch = "x86_64")]
    let peb = get_peb(0x60);

    #[cfg(target_arch = "x86")]
    let peb = get_peb(0x30);

    unsafe {
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

fn breakpoint_hardware() {
    let mut ctx = CONTEXT::default();

    ctx.ContextFlags = CONTEXT_DEBUG_REGISTERS_AMD64;

    unsafe { GetThreadContext(GetCurrentThread(), &mut ctx).unwrap_or_else(|e| {
        println!("[!] GetThreadContext Failed With Error: {e}");
    }) };

    if ctx.Dr0 != 0 || ctx.Dr1 != 0 || ctx.Dr2 != 0 || ctx.Dr3 != 0 {
        println!("[!] Debugger Detected! [4]");
    }
}


// Function to recover PEB
fn get_peb(offset: u64) -> *const PEB {
    let value: u64;

    unsafe {
        asm!(
            "mov {0}, gs:[{1}]",
            out(reg) value,
            in(reg) offset,
            options(nostack, nomem)
        )
    }

    value as *const PEB
}
