use windows::Win32::Foundation::{GetLastError, HANDLE};
use windows::Win32::System::Threading::{CreateEventA, CreateMutexA, CreateSemaphoreA};
use windows::core::{s, Result};

fn main() -> Result<()> {
    unsafe {
        events()?;
        
        // If the event already exists (`GetLastError().is_err()`), it prints a custom message
        // indicating that another instance of the malware might already be running.
        if GetLastError().is_err() {
            println!("[*] MALWARE RUNNING");
        }
    }

    Ok(())
}

/// Creates a named mutex.
#[allow(dead_code)]
fn mutex() -> Result<HANDLE> { 
    unsafe { CreateMutexA(None, false, s!("MalwareA")) }
}

/// Creates a named semaphore.
#[allow(dead_code)]
fn semaphore() -> Result<HANDLE> {
    unsafe { CreateSemaphoreA(None, 10, 10, s!("MalwareA")) } 
}

/// Creates a named event object.
fn events() -> Result<HANDLE> {
    unsafe { CreateEventA(None, false, false, s!("MalwareA")) }
}