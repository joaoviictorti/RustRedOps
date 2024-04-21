#![no_std]
#![no_main]

mod export;

use winapi::um::winuser::{IsDialogMessageW, IsWindowUnicode, MessageBoxA};
use winapi::um::errhandlingapi::GetLastError;
use core::ptr::null_mut;
use winapi::um::synchapi::SetCriticalSectionSpinCount;
use winapi::um::processthreadsapi::ExitProcess;

static mut COUNTER: usize = 0;

unsafe fn fake_api() {
    // You can add more apis if you like.
    GetLastError();
    IsDialogMessageW(null_mut(), null_mut());
    SetCriticalSectionSpinCount(null_mut(), 0);
    MessageBoxA(null_mut(), null_mut(), null_mut(), 0);
    IsWindowUnicode(null_mut());
}

fn maybe_call() {
    unsafe {
        // Increases COUNTER in a way that is not direct
        COUNTER += 1 + (COUNTER % 3);

        // Condition that will never be true, but depends on the value of COUNTER
        if COUNTER > 1000 {
            fake_api();
        }
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    maybe_call();

    unsafe {
        ExitProcess(0);
        loop {}
    }
}