#![allow(unused_must_use)]

use sysinfo::System;
use windows::core::{s, PSTR};
use windows::Win32::System::{
    SystemInformation::{
        GetSystemInfo, GlobalMemoryStatusEx, MEMORYSTATUSEX, SYSTEM_INFO
    },
    Registry::{
        RegCloseKey, RegOpenKeyExA, RegQueryInfoKeyA, HKEY, HKEY_LOCAL_MACHINE, KEY_READ
    },
};

fn main() {
    verify_usb();
    verify_ram();
    verify_cpu();
    verify_processes();
}

///
/// Function that performs a check on the CPU to find out how many processors the computer contains.
///
fn verify_cpu() {
    let mut info: SYSTEM_INFO = SYSTEM_INFO::default();

    unsafe {
        GetSystemInfo(&mut info);
    }

    if info.dwNumberOfProcessors < 2 {
        println!("[*] Possibly a virtualised environment")
    }
}

///
/// Function that performs a check of the current physical memory in bytes and in it we are checking if it is greater than or equal to two gigabytes in bytes.
///
fn verify_ram() {
    let mut info: MEMORYSTATUSEX = MEMORYSTATUSEX::default();
    info.dwLength = std::mem::size_of::<MEMORYSTATUSEX>() as u32;

    unsafe {
        GlobalMemoryStatusEx(&mut info).expect("GlobalMemoryStatusEx Failed");

        if info.ullTotalPhys <= 2 * 1073741824 {
            println!("[*] Possibly a virtualised environment")
        }
    }
}

///
/// The SYSTEM\ControlSet001\Enum\USBSTOR directory in the Windows Registry is a specific location where the operating system
/// stores information about USB storage devices that have been connected to the computer.
/// 
/// Possibly if the computer didn't have 2 usb mounted, it may be in a virtualised environment
fn verify_usb() {
    let mut h_key: HKEY = HKEY::default();
    let mut usb_number: u32 = 0;
    let mut class_name_buffer = [0u8; 256];
    let mut class_name_length = class_name_buffer.len() as u32;

    unsafe {
        let status = RegOpenKeyExA(
            HKEY_LOCAL_MACHINE,
            s!("SYSTEM\\ControlSet001\\Enum\\USBSTOR"),
            0,
            KEY_READ,
            &mut h_key,
        );

        if status.is_err() {
            println!("RegOpenKeyExA Failed");
            return;
        }

        let status = RegQueryInfoKeyA(
            h_key,
            PSTR(class_name_buffer.as_mut_ptr()),
            Some(&mut class_name_length),
            None,
            Some(&mut usb_number),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );

        if status.is_err() {
            println!("RegQueryInfoKeyA Failed");
            return;
        }

        if usb_number < 2 {
            println!("[*] Possibly a virtualised environment");
            return;
        }

        RegCloseKey(h_key);
    }
}

////
/// Check if the environment can be sandboxed through the number of processes running
/// 
fn verify_processes() {
    let mut system = System::new_all();
    system.refresh_all();

    let number_processes = system.processes().len();

    if number_processes <= 50 {
        println!("[*] Possibly a sandbox environment");
    }
}