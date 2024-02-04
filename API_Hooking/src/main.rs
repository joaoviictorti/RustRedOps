use std::{mem::size_of, os::raw::c_void, ptr::copy, ffi::{c_char, CStr}};
use windows::{
    core::{s, w},
    Win32::{
        Foundation::HWND,
        System::LibraryLoader::{GetProcAddress, LoadLibraryA},
        System::Memory::{VirtualProtect, PAGE_EXECUTE_READWRITE, PAGE_PROTECTION_FLAGS},
        UI::WindowsAndMessaging::{MessageBoxA, MessageBoxW, MESSAGEBOX_STYLE},
        UI::WindowsAndMessaging::{MB_OK, MESSAGEBOX_RESULT},
    },
};

extern "system" fn my_message_box_a(
    hwnd: HWND,
    lp_text: *const c_char,
    lp_caption: *const c_char,
    u_type: MESSAGEBOX_STYLE,
) -> MESSAGEBOX_RESULT {
    let c_str_text = unsafe { CStr::from_ptr(lp_text) };
    let text = c_str_text.to_string_lossy();

    let c_str_caption = unsafe { CStr::from_ptr(lp_caption) };
    let caption = c_str_caption.to_string_lossy();

    println!("[+] Parameters sent by the original function:");
    println!("\t - text    : {}", text);
    println!("\t - caption : {}", caption);

    unsafe { MessageBoxW(hwnd, w!("HOOK"), w!("ENABLED!"), u_type) }
}

struct Hook {
    #[cfg(target_arch = "x86_64")]
    bytes_original: [u8; 13],
    #[cfg(target_arch = "x86")]
    bytes_original: [u8; 7],
    function_run: *mut c_void,
    function_hook: *mut c_void,
}

impl Hook {
    fn new(function_run: *mut c_void, function_hook: *mut c_void) -> Self {
        Hook {
            #[cfg(target_arch = "x86_64")]
            bytes_original: [0; 13],
            #[cfg(target_arch = "x86")]
            bytes_original: [0; 7],
            function_run,
            function_hook,
        }
    }

    fn initialize(&mut self, trampoline: &[u8], old_protect: &mut PAGE_PROTECTION_FLAGS) -> bool {
        unsafe {
            copy(
                self.function_hook,
                self.bytes_original.as_mut_ptr() as *mut c_void,
                trampoline.len(),
            );

            let result = VirtualProtect(
                self.function_hook,
                trampoline.len(),
                PAGE_EXECUTE_READWRITE,
                old_protect,
            );
            if result.is_err() {
                println!("[!] VirtualProtect Failed With Error {:?}", result.err());
                return false;
            }
        }
        true
    }

    fn install_hook(&self, trampoline: &mut [u8]) {

        unsafe {
            copy(
                &self.function_run as  *const _ as *const c_void,
                trampoline[2..].as_mut_ptr() as *mut c_void,
                size_of::<*mut c_void>(),
            );

            copy(
                trampoline.as_ptr() as *const c_void,
                self.function_hook,
                trampoline.len(),
            );
        }
    }
}

fn main() {
    #[cfg(target_arch = "x86_64")]
    let mut trampoline: [u8; 13] = [
        0x49, 0xBA, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // mov r10, function
        0x41, 0xFF, 0xE2, // jmp r10
    ];

    #[cfg(target_arch = "x86")]
    let mut trampoline: [u8; 7] = [
        0xB8, 0x00, 0x00, 0x00, 0x00, // mov eax, function
        0xFF, 0xE0, // jmp eax
    ];

    let hmodule = unsafe { LoadLibraryA(s!("user32.dll")).unwrap() };
    let func = unsafe { GetProcAddress(hmodule, s!("MessageBoxA")).unwrap() };

    let mut hook = Hook::new(my_message_box_a as *mut c_void, func as *mut c_void);

    let mut oldprotect = PAGE_PROTECTION_FLAGS(0);

    if hook.initialize(&mut trampoline, &mut oldprotect) {
        hook.install_hook(&mut trampoline);
    } else {
        println!("[!] Failed to Apply Hook!");
        return;
    }

    unsafe {

        MessageBoxA(HWND(0), s!("Test Message"), s!("Test"), MB_OK);

        println!("[+] Hook disabled");
        copy(
            hook.bytes_original.as_ptr(),
            hook.function_hook as *mut u8,
            trampoline.len(),
        );

        let mut d_old_protect = PAGE_PROTECTION_FLAGS(0);
        let protection_address = VirtualProtect(hook.function_hook, trampoline.len(), oldprotect, &mut d_old_protect);

        if protection_address.is_err() {
            println!("[!] VirtualProtect Failed With Error {:?}", protection_address.err());
            return ;
        }

        MessageBoxA(HWND(0), s!("Test Message"), s!("Test"), MB_OK);
    }

    println!("[+] Finish");
}
