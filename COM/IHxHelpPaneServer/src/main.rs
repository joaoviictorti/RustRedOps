#![allow(non_snake_case, non_camel_case_types)]

use core::ffi::c_void;
use windows::{
    core::{w, PCWSTR},
    Win32::System::Com::{
        CLSIDFromString, CoCreateInstance, CoInitializeEx,
        CoUninitialize, CLSCTX_ALL, COINIT_MULTITHREADED
    },
};

fn main() -> windows::core::Result<()> {
    unsafe {
        // Initializes the COM environment
        CoInitializeEx(None, COINIT_MULTITHREADED).ok()?;

        let clsid = CLSIDFromString(w!("{8cec58ae-07a1-11d9-b15e-000d56bfe6ee}"))?;
        let helpserver: IHxHelpPaneServer = CoCreateInstance(&clsid, None, CLSCTX_ALL)?;

        // Execute Command
        helpserver.Execute(w!("file:///C:/WINDOWS/System32/cmd.exe"));

        // Cleanup COM after operations complete
        CoUninitialize();
    }

    Ok(())
}

windows::core::imp::define_interface!(IHxHelpPaneServer, IHxHelpPaneServer_Vtbl, 0x8cec592c_07a1_11d9_b15e_000d56bfe6ee);
windows::core::imp::interface_hierarchy!(IHxHelpPaneServer, windows::core::IUnknown);

#[repr(C)]
pub struct IHxHelpPaneServer_Vtbl {
    pub base__: windows::core::IUnknown_Vtbl,
    pub DisplayTask: unsafe fn(this: *mut c_void, pcUrl: PCWSTR),
    pub DisplayContents: unsafe fn(this: *mut c_void, pcUrl: PCWSTR),
    pub DisplaySearchResults: unsafe fn(this: *mut c_void, pcUrl: PCWSTR),
    pub Execute: unsafe fn(this: *mut c_void, pcUrl: PCWSTR),
}

impl IHxHelpPaneServer {
    pub fn DisplayTask(&self, pcUrl: PCWSTR) {
        unsafe { (windows::core::Interface::vtable(self).DisplayTask)(windows::core::Interface::as_raw(self), pcUrl) }
    }

    pub fn DisplayContents(&self, pcUrl: PCWSTR) {
        unsafe { (windows::core::Interface::vtable(self).DisplayContents)(windows::core::Interface::as_raw(self), pcUrl) }
    }

    pub fn DisplaySearchResults(&self, pcUrl: PCWSTR) {
        unsafe { (windows::core::Interface::vtable(self).DisplaySearchResults)(windows::core::Interface::as_raw(self), pcUrl) }
    }

    pub fn Execute(&self, pcUrl: PCWSTR) {
        unsafe { (windows::core::Interface::vtable(self).Execute)(windows::core::Interface::as_raw(self), pcUrl) }
    }
}

impl core::ops::Deref for IHxHelpPaneServer {
    type Target = windows::core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}