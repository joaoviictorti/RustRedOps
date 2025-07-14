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

        // Information to use in DLL injection
        let pid = args[1].parse::<u32>()?;
        let path = &args[2];
        let dll = path.encode_utf16().collect::<Vec<u16>>();

        // Resolve the address of LoadLibraryW from kernel32.dll
        let load_library = GetProcAddress(GetModuleHandleW(w!("Kernel32"))?, s!("LoadLibraryW"));

        // Open a handle to the remote process
        let hprocess = OpenProcess(PROCESS_ALL_ACCESS, false, pid)?;

        // Allocate memory in the remote process to store the DLL path
        let address = VirtualAllocEx(
            hprocess,
            None,
            dll.len() * size_of::<u16>(),
            MEM_RESERVE | MEM_COMMIT,
            PAGE_READWRITE,
        );

        if address.is_null() {
            eprintln!("VirtualAllocEx returned null");
            return Ok(())
        }

        // Write the DLL path into the allocated memory in the remote process
        WriteProcessMemory(
            hprocess,
            address,
            dll.as_ptr().cast(),
            dll.len() * size_of::<u16>(),
            None,
        )?;

        // Create a remote thread to call LoadLibraryW with the DLL path as argument
        let hthread = CreateRemoteThread(
            hprocess,
            None,
            0,
            Some(transmute(load_library)),
            Some(address),
            0,
            None,
        )?;

        CloseHandle(hthread)?;
        CloseHandle(hprocess)?;
    }

    Ok(())
}
