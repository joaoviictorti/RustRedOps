use std::{
    ffi::{c_void, CStr},
    slice::{from_raw_parts, from_raw_parts_mut},
};
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

/// Custom replacement for `MessageBoxA` that intercepts parameters and shows a Unicode message box.
///
/// # Arguments
/// 
/// * `hwnd` - Handle to the owner window.
/// * `lp_text` - Pointer to the message text (ANSI).
/// * `lp_caption` - Pointer to the message box caption (ANSI).
/// * `u_type` - Message box style flags.
///
/// # Returns
/// 
/// * Returns the result from the `MessageBoxW` function.
extern "system" fn my_message_box_a(
    hwnd: HWND,
    lp_text: *const i8,
    lp_caption: *const i8,
    u_type: MESSAGEBOX_STYLE,
) -> MESSAGEBOX_RESULT {
    let text = unsafe { CStr::from_ptr(lp_text).to_string_lossy() };
    let caption = unsafe { CStr::from_ptr(lp_caption).to_string_lossy() };

    println!("[+] Parameters sent by the original function:");
    println!("\t - text    : {}", text);
    println!("\t - caption : {}", caption);

    unsafe { MessageBoxW(hwnd, w!("HOOK"), w!("ENABLED!"), u_type) }
}

/// Structure to manage function hooking and restoration.
struct Hook {
    /// Backup of original bytes overwritten by the trampoline.
    #[cfg(target_arch = "x86_64")]
    bytes_original: [u8; 13],

    /// Backup of original bytes overwritten by the trampoline (x86).
    #[cfg(target_arch = "x86")]
    bytes_original: [u8; 7],

    /// Pointer to the custom function to redirect execution to.
    function_run: *mut c_void,

    /// Pointer to the target function to be hooked.
    function_hook: *mut c_void,
}

impl Hook {
    /// Creates a new hook structure linking a target function with the replacement.
    ///
    /// # Arguments
    /// 
    /// * `function_run` - Pointer to the new function.
    /// * `function_hook` - Pointer to the function to be overwritten.
    ///
    /// # Returns
    /// 
    /// * A new `Hook` instance.
    fn new(function_run: *mut c_void, function_hook: *mut c_void) -> Self {
        Self {
            #[cfg(target_arch = "x86_64")]
            bytes_original: [0; 13],
            #[cfg(target_arch = "x86")]
            bytes_original: [0; 7],
            function_run,
            function_hook,
        }
    }

    /// Initializes the hook by changing memory protections and saving original bytes.
    ///
    /// # Arguments
    /// 
    /// * `trampoline` - Trampoline code to be written.
    /// * `old_protect` - Variable to store the old protection flags.
    ///
    /// # Returns
    /// 
    /// * `true` if initialization succeeded, `false` otherwise.
    fn initialize(&mut self, trampoline: &[u8], old_protect: &mut PAGE_PROTECTION_FLAGS) -> bool {
        unsafe {
            let result = VirtualProtect(self.function_hook, trampoline.len(), PAGE_EXECUTE_READWRITE, old_protect);
            if result.is_err() {
                eprintln!("[!] VirtualProtect Failed With Error {:?}", result.err());
                return false;
            }

            let bytes = from_raw_parts(self.function_hook.cast::<u8>(), trampoline.len());
            self.bytes_original.copy_from_slice(bytes);
        }

        true
    }

    /// Writes the trampoline and redirects the execution flow.
    ///
    /// # Arguments
    /// 
    /// * `trampoline` - Mutable slice representing the JMP stub.
    fn install_hook(&self, trampoline: &mut [u8]) {
        unsafe {
            let dst = trampoline[2..].as_mut_ptr();
            let src = &self.function_run as *const *mut c_void as *const u8;

            let trampoline_bytes = from_raw_parts_mut(dst, size_of::<*const c_void>());
            let func_bytes = from_raw_parts(src, size_of::<*const c_void>());
            trampoline_bytes.copy_from_slice(func_bytes);

            let dst_code = from_raw_parts_mut(self.function_hook.cast::<u8>(), trampoline.len());
            dst_code.copy_from_slice(trampoline);
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

    // Load target DLL and resolve target function
    let hmodule = unsafe { LoadLibraryA(s!("user32.dll")).unwrap() };
    let func = unsafe { GetProcAddress(hmodule, s!("MessageBoxA")).unwrap() };

    // Initialize hook and apply trampoline
    let mut oldprotect = PAGE_PROTECTION_FLAGS(0);
    let mut hook = Hook::new(my_message_box_a as *mut c_void, func as *mut c_void);
    if hook.initialize(&mut trampoline, &mut oldprotect) {
        hook.install_hook(&mut trampoline);
    } else {
        eprintln!("[!] Failed to Apply Hook!");
        return;
    }

    unsafe {
        // Trigger the hook
        MessageBoxA(HWND(0), s!("Test Message"), s!("Test"), MB_OK);
        println!("[+] Hook disabled");

        // Restore original bytes
        let restore_target = from_raw_parts_mut(hook.function_hook.cast::<u8>(), trampoline.len());
        restore_target.copy_from_slice(&hook.bytes_original);

        // Restore memory protection
        let mut old_protect = PAGE_PROTECTION_FLAGS(0);
        let addr = VirtualProtect(hook.function_hook, trampoline.len(), oldprotect, &mut old_protect);
        if addr.is_err() {
            eprintln!("[!] VirtualProtect Failed With Error {:?}", addr.err());
            return;
        }

        // Call again with restored hook
        MessageBoxA(HWND(0), s!("Test Message"), s!("Test"), MB_OK);
    }

    println!("[+] Finish");
}