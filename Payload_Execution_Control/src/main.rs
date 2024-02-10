use windows::Win32::Foundation::GetLastError;
use windows::Win32::System::Threading::{CreateEventA, CreateMutexA, CreateSemaphoreA};
use windows::core::s;

fn main() {
    unsafe {
        events();
        if GetLastError().is_err() {
            println!("{}", "MALWARE RUNNING");
        }
    }
}

unsafe fn mutex() { 
    let _ = CreateMutexA(None, false, s!("MalwareA"));
}

unsafe fn semaphore() {
    let _ = CreateSemaphoreA(None, 10, 10, s!("MalwareA"));
}

unsafe fn events() {
    let _ = CreateEventA(None, false, false, s!("MalwareA"));
}