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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let args = std::env::args().collect::<Vec<String>>();
        if args.len() != 3 {
            eprintln!(".\\Dllinjection_rs.exe <pid> <path>");
            return Ok(());
        }

        let pid = args[1].parse::<u32>()?;

        let path = &args[2];
        let dll = path.encode_utf16().collect::<Vec<u16>>();

        let proc_address = GetProcAddress(GetModuleHandleW(w!("Kernel32"))?, s!("LoadLibraryW"));
        let hprocess = OpenProcess(PROCESS_ALL_ACCESS, false, pid)?;
        let address = VirtualAllocEx(
            hprocess,
            None,
            dll.len() * size_of::<u16>(),
            MEM_RESERVE | MEM_COMMIT,
            PAGE_READWRITE,
        );

        if address.is_null() {
            eprintln!("Address is null");
            return Ok(())
        }

        WriteProcessMemory(
            hprocess,
            address,
            dll.as_ptr().cast(),
            dll.len() * size_of::<u16>(),
            None,
        )?;

        let hthread = CreateRemoteThread(
            hprocess,
            None,
            0,
            Some(transmute(proc_address)),
            Some(address),
            0,
            None,
        )?;

        CloseHandle(hprocess)?;
        CloseHandle(hthread)?;
    }

    Ok(())
}
