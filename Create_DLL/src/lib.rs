use windows::Win32::Foundation::{BOOL, HINSTANCE, HWND};
use windows::core::s;
use windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH;
use windows::Win32::UI::WindowsAndMessaging::MessageBoxA;

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
extern "system" fn DllMain(hinstance: HINSTANCE, reason: u32, _: *mut std::ffi::c_void) -> BOOL {
    match reason {
        DLL_PROCESS_ATTACH => {
            unsafe {
                MessageBoxA(HWND(0), s!("Hello"), s!("World"), Default::default());
            }
        },
        _ => {}
    }

    true.into()
}
