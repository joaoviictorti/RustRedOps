#![no_std]
#![no_main]

mod export;
use core::ptr::null_mut;
use windows_sys::Win32::Foundation::GetLastError;
use windows_sys::Win32::UI::WindowsAndMessaging::{IsDialogMessageW, MessageBoxA, IsWindowUnicode};
use windows_sys::Win32::System::Threading::SetCriticalSectionSpinCount;

static mut COUNTER: usize = 0;

// You can add more apis if you like.
fn fake_api() {
    unsafe {
        GetLastError();
        IsDialogMessageW(null_mut(), null_mut());
        SetCriticalSectionSpinCount(null_mut(), 0);
        MessageBoxA(null_mut(), null_mut(), null_mut(), 0);
        IsWindowUnicode(null_mut());
    }
}

fn call() {
    unsafe {
        // Increases COUNTER in a way that is not direct
        COUNTER += 1 + (COUNTER % 3);

        // Condition that will never be true, but depends on the value of COUNTER
        if COUNTER > 1000 {
            fake_api();
        }
    }
}

#[no_mangle]
fn main() -> u8 {
    call();
    0
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
