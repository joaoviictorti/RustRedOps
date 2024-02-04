use std::mem::{size_of, transmute};
use windows::{
    core::{s, w},
    Win32::{
        Foundation::CloseHandle,
        System::{
            Diagnostics::Debug::WriteProcessMemory,
            LibraryLoader::{GetModuleHandleW, GetProcAddress},
            Memory::{VirtualAllocEx, MEM_COMMIT, MEM_RESERVE, PAGE_READWRITE},
            Threading::{CreateRemoteThread, OpenProcess, PROCESS_ALL_ACCESS},
        },
    },
};

fn main() {
    unsafe {
        let args: Vec<String> = std::env::args().collect();
        if args.len() != 3 {
            println!(".\\Dllinjection_rs.exe <pid> <path>");
            return;
        }

        let pid = args[1].parse::<u32>().unwrap_or_else(|_e| {
            panic!("[!] PID error format");
        });

        let path = &args[2];
        let dll: Vec<u16> = path.encode_utf16().collect();

        let proc_address = GetProcAddress(GetModuleHandleW(w!("Kernel32")).unwrap(), s!("LoadLibraryW"));

        let hprocess = OpenProcess(PROCESS_ALL_ACCESS, false, pid).unwrap_or_else(|e| {
            panic!("[!] OpenProcess Failed With Error: {e}");
        });

        let address = VirtualAllocEx(
            hprocess,
            None,
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
        ).unwrap_or_else(|e| {
            panic!("[!] WriteProcessMemory Failed With Error: {e}");
        });

        let hthread = CreateRemoteThread(
            hprocess,
            None,
            0,
            Some(transmute(proc_address)),
            Some(address),
            0,
            None,
        ).unwrap_or_else(|e| {
            panic!("[!] CreateRemoteThread Failed With Error: {e}");
        });

        CloseHandle(hprocess);
        CloseHandle(hthread);
    }
}
