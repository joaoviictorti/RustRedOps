#![allow(unused_variables)]

use windows::{
    core::*,
    Win32::{
        self,
        Foundation::{BOOL, HWND},
        System::Diagnostics::Debug::ActiveScript::{
            IActiveScriptError, IActiveScriptSite, IActiveScriptSiteWindow,
            IActiveScriptSiteWindow_Impl, IActiveScriptSite_Impl, SCRIPTSTATE,
        },
    },
};

#[implement(IActiveScriptSite, IActiveScriptSiteWindow)]
pub struct ScriptSite;

impl ScriptSite {
    pub fn new() -> Self {
        Self
    }
}

impl IActiveScriptSiteWindow_Impl for ScriptSite_Impl {
    fn GetWindow(&self) -> Result<HWND> {
        Ok(HWND::default())
    }

    fn EnableModeless(&self, _: BOOL) -> Result<()> {
        Ok(())
    }
}

impl IActiveScriptSite_Impl for ScriptSite_Impl {
    fn GetLCID(&self) -> Result<u32> {
        Ok(1033)
    }

    fn GetItemInfo(
        &self,
        pstrname: &PCWSTR,
        dwreturnmask: u32,
        ppiunkitem: *mut Option<windows_core::IUnknown>,
        ppti: *mut Option<Win32::System::Com::ITypeInfo>,
    ) -> Result<()> {
        Ok(())
    }

    fn GetDocVersionString(&self) -> Result<BSTR> {
        Ok(BSTR::new())
    }

    fn OnScriptTerminate(
        &self,
        pvarresult: *const VARIANT,
        pexcepinfo: *const Win32::System::Com::EXCEPINFO,
    ) -> Result<()> {
        Ok(())
    }

    fn OnStateChange(&self, ssscriptstate: SCRIPTSTATE) -> Result<()> {
        Ok(())
    }

    fn OnScriptError(&self, pscripterror: Option<&IActiveScriptError>) -> Result<()> {
        Ok(())
    }

    fn OnEnterScript(&self) -> Result<()> {
        Ok(())
    }

    fn OnLeaveScript(&self) -> Result<()> {
        Ok(())
    }
}
