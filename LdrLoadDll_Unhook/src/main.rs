use std::ffi::{CString, OsString};
use std::os::windows::ffi::OsStrExt;
use std::ptr;
use std::ptr::null_mut;

use winapi::shared::ntdef::{LPCSTR, PULONG, PUNICODE_STRING, PVOID, UNICODE_STRING, WCHAR};
use windows::{
    Win32::Foundation::*,
    Win32::System::LibraryLoader::*,
    Win32::System::Memory::*,
};
use windows::core::s;

pub type PWSTR = *mut WCHAR;

type PNewLdrLoadDll = unsafe extern "system" fn (
    DllPath: PWSTR,
    DllCharacteristics: PULONG,
    DllName: PUNICODE_STRING,
    DllHandle: *mut PVOID
) -> NTSTATUS;
type PMyMessageBoxA = unsafe extern "system" fn(HWND, LPCSTR, LPCSTR, u32) -> i32;

unsafe fn copy_memory(destination: *mut u8, source: *const u8, length: usize) {
    let mut d = destination;
    let mut s = source;

    for _ in 0..length {
        *d = *s;
        d = d.add(1);
        s = s.add(1);
    }
}

fn main() {
    let hmodule = unsafe { LoadLibraryA(s!("ntdll.dll")).unwrap() };
    let orign_ldr_load_dll = unsafe { GetProcAddress(hmodule, s!("LdrLoadDll")).unwrap() };

    let jmp_addr: *const () = (orign_ldr_load_dll as usize + 0x5) as *const ();

    let orgin:[u8;5]=[0x48,0x89,0x5c,0x24,0x10];
    let jump_prelude: [u8; 2] = [0x49, 0xBB];
    let jump_epilogue: [u8; 4] = [0x41, 0xFF, 0xE3, 0xC3];
    let trampoline=unsafe{
        VirtualAlloc(None, 19, MEM_RESERVE | MEM_COMMIT, PAGE_READWRITE)
    };
    unsafe{
        let addr_ptr: *const u8 = std::ptr::addr_of!(jmp_addr) as *const u8;
        copy_memory(trampoline as *mut u8,orgin.as_ptr(),5);
        copy_memory(trampoline.add(5) as *mut u8,jump_prelude.as_ptr(),2);
        copy_memory(trampoline.add(5).add(2) as *mut u8,addr_ptr,8);
        copy_memory(trampoline.add(5).add(2).add(8) as *mut u8,jump_epilogue.as_ptr(),4);
        let mut oldprotect = PAGE_PROTECTION_FLAGS(0);
        VirtualProtect(trampoline, 30, PAGE_EXECUTE_READ, &mut oldprotect).expect("TODO: panic message");
        let ldr_loadr_dll: PNewLdrLoadDll = std::mem::transmute(trampoline);
        
        // Convert DLL name to wide string
        let user32_dll_name: Vec<u16> = OsString::from("user32.dll")
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();

        // Prepare UNICODE_STRING for DLL name
        let mut user32_dll_unicode = UNICODE_STRING {
            Length: ((user32_dll_name.len() - 1) * 2) as u16,
            MaximumLength: (user32_dll_name.len() * 2) as u16,
            Buffer: user32_dll_name.as_ptr() as *mut _,
        };
        let mut user32handle = null_mut();
        
        //get user32.dll
        ldr_loadr_dll(ptr::null_mut(), 0 as PULONG, &mut user32_dll_unicode, &mut  user32handle);
        let user32handle: HMODULE = unsafe { std::mem::transmute(user32handle) };

        //MessageBoxA
        let my_message_box_a_addr =   GetProcAddress(user32handle, s!("MessageBoxA")).unwrap() ;
        let MyMessageBoxA:PMyMessageBoxA=std::mem::transmute(my_message_box_a_addr);
        let text_cstring = CString::new("Hello, World!").unwrap();
        let caption_cstring = CString::new("Title").unwrap();

        let hwnd = HWND(0);
        MyMessageBoxA(hwnd, text_cstring.as_ptr(), caption_cstring.as_ptr(), 0 as u32);
    };
}
