use std::{
    mem::{size_of, transmute},
    process::exit,
    ptr::{null, null_mut},
};

use windows::core::{s, w};
use windows::Win32::Foundation::CloseHandle;
use windows::Win32::System::Diagnostics::Debug::WriteProcessMemory;
use windows::Win32::System::LibraryLoader::{GetModuleHandleW, GetProcAddress};
use windows::Win32::System::Memory::{VirtualAllocEx, MEM_COMMIT, MEM_RESERVE, PAGE_READWRITE};
use windows::Win32::System::Threading::{CreateRemoteThread, OpenProcess, PROCESS_ALL_ACCESS};

fn main() {
    unsafe {
        let args: Vec<String> = std::env::args().collect();
        if args.len() != 3 {
            println!(".\\Dllinjection_rs.exe <pid> <path>");
            exit(0);
        }

        let pid = args[1].parse::<u32>().unwrap_or_else(|_e| {
            println!("[!] PID error format");
            exit(0)
        });

        let path = &args[2];
        let dll: Vec<u16> = path.encode_utf16().collect();

        let proc_address = GetProcAddress(
            GetModuleHandleW(w!("Kernel32")).unwrap(),
            s!("LoadLibraryW"),
        );

        let hprocess = OpenProcess(PROCESS_ALL_ACCESS, false, pid).unwrap_or_else(|e| {
            println!("[!] OpenProcess Failed With Error: {e}");
            exit(-1);
        });

        let address = VirtualAllocEx(
            hprocess,
            Some(null_mut()),
            dll.len() * size_of::<u16>(),
            MEM_RESERVE | MEM_COMMIT,
            PAGE_READWRITE,
        );

        WriteProcessMemory(
            hprocess,
            address,
            dll.as_ptr() as _,
            dll.len() * size_of::<u16>(),
            None,
        )
        .unwrap_or_else(|e| {
            eprintln!("[!] WriteProcessMemory Failed With Error: {e}");
            CloseHandle(hprocess);
            exit(-1);
        });

        let hthread = CreateRemoteThread(
            hprocess,
            Some(null()),
            0,
            Some(transmute(proc_address)),
            Some(address),
            0,
            Some(null_mut()),
        )
        .unwrap_or_else(|e| {
            eprintln!("[!] CreateRemoteThread Failed With Error: {e}");
            CloseHandle(hprocess);
            exit(-1);
        });

        CloseHandle(hprocess);
        CloseHandle(hthread);
    }
}
