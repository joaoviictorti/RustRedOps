#![no_std]

use winapi::{
    km::wdm::{DbgPrint, DRIVER_OBJECT},
    shared::{ntdef::{NTSTATUS, UNICODE_STRING}, ntstatus::STATUS_SUCCESS},
};

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "system" fn driver_entry(driver_object: &mut DRIVER_OBJECT, _: &UNICODE_STRING) -> NTSTATUS {
    driver_object.DriverUnload = Some(driver_unload);
    
    unsafe {
        DbgPrint("Hello World!\0".as_ptr() as _,);
    }

    STATUS_SUCCESS
}

pub extern "system" fn driver_unload(_driver: &mut DRIVER_OBJECT) {
    unsafe {
        DbgPrint("GoodBye!\0".as_ptr() as _);
    }
}
