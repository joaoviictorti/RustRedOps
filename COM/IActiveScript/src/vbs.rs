// #![allow(dead_code)]

use crate::data::ScriptSite;
use std::ptr::null_mut;
use windows::{
    core::{w, Interface, Result, PCWSTR},
    Win32::System::{Com::*, Diagnostics::Debug::ActiveScript::*},
};

/// A wrapper for the Microsoft VBScript scripting engine using COM interfaces.
pub struct VBScript {
    /// Handle to the scripting engine (`IActiveScript`).
    engine: IActiveScript,

    /// Interface for parsing and evaluating VBScript code (`IActiveScriptParse64`).
    parse: IActiveScriptParse64,
}

impl VBScript {
    /// Creates and initializes a new [`VBScript`] engine instance.
    pub fn new() -> Result<Self> {
        unsafe {
            // Initializes the COM environment
            CoInitializeEx(None, COINIT_MULTITHREADED).ok()?;

            // Configures VBScript as the language
            let guid = CLSIDFromProgID(w!("VBScript"))?;
            let engine: IActiveScript = CoCreateInstance(&guid, None, CLSCTX_ALL)?;

            // Sets up the ScriptSite
            let site: IActiveScriptSite = ScriptSite.into();
            engine.SetScriptSite(&site)?;

            // Gets the parsing interface
            let parse = engine.cast::<IActiveScriptParse64>()?;
            parse.InitNew()?;

            Ok(Self { engine, parse })
        }
    }

    /// Executes a VBScript snippet provided as a UTF-8 Rust string.
    ///
    /// # Arguments
    ///
    /// * `script` - A string slice containing the VBScript code to run
    pub fn run(&self, script: &str) -> Result<()> {
        unsafe {
            // Activates the script state
            self.engine.SetScriptState(SCRIPTSTATE_CONNECTED)?;

            // Converts the script code to PCWSTR
            let wide = script.encode_utf16().chain(Some(0)).collect::<Vec<u16>>();
            let code = PCWSTR::from_raw(wide.as_ptr());

            // Executes the script
            self.parse.ParseScriptText(
                code,
                None,
                None,
                None,
                0,
                0,
                SCRIPTTEXT_ISVISIBLE,
                null_mut(),
                null_mut(),
            )?;

            Ok(())
        }
    }
}

impl Drop for VBScript {
    fn drop(&mut self) {
        unsafe {
            self.engine.Close().ok();
        }
    }
}
