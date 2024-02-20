use std::arch::asm;
use sysinfo::System;
use windows::Win32::System::Diagnostics::Debug::{
    GetThreadContext, IsDebuggerPresent, CONTEXT, CONTEXT_DEBUG_REGISTERS_AMD64
};
use windows::Win32::System::Threading::{GetCurrentThread, PEB, TEB};
use windows::Win32::System::Kernel::NT_TIB;

fn main() {
    is_debugger_present();
    is_debugger_peb();
    process_list();
    breakpoint_hardware();
    std::thread::sleep(std::time::Duration::from_secs(20000));
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
unsafe fn get_peb() -> *mut PEB {
    let teb_offset = ntapi::FIELD_OFFSET!(NT_TIB, Self_) as u32;

    #[cfg(target_arch = "x86_64")]
    {
        let teb = __readgsqword(teb_offset) as *mut TEB;
        return (*teb).ProcessEnvironmentBlock;
    }

    #[cfg(target_arch = "x86")]
    {
        let teb = __readfsdword(teb_offset) as *mut TEB;
        return (*teb).ProcessEnvironmentBlock;
    }
}

#[cfg(target_arch = "x86_64")]
unsafe fn __readgsqword(offset: u32) -> u64 {
    let output: u64;
    asm!(
        "mov {}, gs:[{:e}]",
        lateout(reg) output,
        in(reg) offset,
        options(nostack, pure, readonly),
    );
    output
}

#[cfg(target_arch = "x86")]
unsafe fn __readfsdword(offset: u32) -> u32 {
    let output: u32;
    asm!(
        "mov {:e}, fs:[{:e}]",
        lateout(reg) output,
        in(reg) offset,
        options(nostack, pure, readonly),
    );
    output
}