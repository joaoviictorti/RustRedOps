#![allow(unused_must_use, non_snake_case)]

use std::{
    ffi::{CString, OsString},
    os::windows::ffi::OsStrExt,
    ptr::{self, null_mut},
};
use winapi::shared::ntdef::{
    LPCSTR, PULONG, PUNICODE_STRING, 
    PVOID, UNICODE_STRING, WCHAR
};
use windows::{
    core::s,
    Win32::{Foundation::*, System::{LibraryLoader::*, Memory::*}},
};

type PWSTR = *mut WCHAR;
type MessageBoxAType = unsafe extern "system" fn(HWND, LPCSTR, LPCSTR, u32) -> i32;
type LdrLoadDllType = unsafe extern "system" fn (
    DllPath: PWSTR,
    DllCharacteristics: PULONG,
    DllName: PUNICODE_STRING,
    DllHandle: *mut PVOID
) -> NTSTATUS;

unsafe fn copy_memory(destination: *mut u8, source: *const u8, length: usize) {
    let mut d = destination;
    let mut s = source;

    for _ in 0..length {
        *d = *s;
        d = d.add(1);
        s = s.add(1);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    unsafe{
        let hmodule = LoadLibraryA(s!("ntdll.dll"))?;
        let orign_ldr_load_dll = GetProcAddress(hmodule, s!("LdrLoadDll"))
                .ok_or("Error retrieving the address of LdrLoadDll")?;
    
        let jmp_addr = (orign_ldr_load_dll as usize + 0x5) as *const ();
        let orgin = [0x48, 0x89, 0x5c, 0x24, 0x10];
        let jump_prelude = [0x49, 0xBB];
        let jump_epilogue  = [0x41, 0xFF, 0xE3, 0xC3];

        let addr_ptr  = std::ptr::addr_of!(jmp_addr) as *const u8;
        let trampoline= VirtualAlloc(None, 19, MEM_RESERVE | MEM_COMMIT, PAGE_READWRITE);
        copy_memory(trampoline.cast(), orgin.as_ptr(), 5);
        copy_memory(trampoline.add(5).cast(), jump_prelude.as_ptr(), 2);
        copy_memory(trampoline.add(5).add(2).cast(), addr_ptr, 8);
        copy_memory(trampoline.add(5).add(2).add(8).cast(), jump_epilogue.as_ptr(), 4);
        
        let mut oldprotect = PAGE_PROTECTION_FLAGS(0);
        VirtualProtect(trampoline, 30, PAGE_EXECUTE_READ, &mut oldprotect)?;

        // Convert DLL name to wide string
        let user32_dll_name = OsString::from("user32.dll")
            .encode_wide()
            .chain(Some(0))
            .collect::<Vec<u16>>();

        // Prepare UNICODE_STRING for DLL name
        let mut user32_dll_unicode = UNICODE_STRING {
            Length: ((user32_dll_name.len() - 1) * 2) as u16,
            MaximumLength: (user32_dll_name.len() * 2) as u16,
            Buffer: user32_dll_name.as_ptr().cast_mut(),
        };
        
        // Get user32.dll
        let LdrLoadDll = std::mem::transmute::<_, LdrLoadDllType>(trampoline);
        let mut user32handle = null_mut();
        LdrLoadDll(ptr::null_mut(), 0 as PULONG, &mut user32_dll_unicode, &mut user32handle);

        // MessageBoxA
        let my_message_box_a_addr = GetProcAddress(HMODULE(user32handle as isize), s!("MessageBoxA"))
            .ok_or("Error retrieving the address of MessageBoxA")?;

        let MessageBoxA = std::mem::transmute::<_, MessageBoxAType>(my_message_box_a_addr);
        MessageBoxA(HWND(0), CString::new("Hello, World!")?.as_ptr(), CString::new("Title")?.as_ptr(), 0);
    }

    Ok(())
}
