#![no_std]

#[cfg(not(test))]
extern crate wdk_panic;

use kernel_log::KernelLogger;
#[cfg(not(test))]
use wdk_alloc::WDKAllocator;

use wdk_sys::{DRIVER_OBJECT, NTSTATUS, PCUNICODE_STRING, STATUS_SUCCESS};

#[cfg(not(test))]
#[global_allocator]
static GLOBAL_ALLOCATOR: WDKAllocator = WDKAllocator;

#[export_name = "DriverEntry"]
pub unsafe extern "system" fn driver_entry(
    driver: &mut DRIVER_OBJECT,
    _registry_path: PCUNICODE_STRING,
) -> NTSTATUS {
    KernelLogger::init(log::LevelFilter::Info).expect("Failed to initialize logger");

    log::info!("Hello World!");

    driver.DriverUnload = Some(driver_unload);

    STATUS_SUCCESS
}

pub unsafe extern "C" fn driver_unload(_driver_object: *mut DRIVER_OBJECT) {
    log::info!("Bye bye!");
}