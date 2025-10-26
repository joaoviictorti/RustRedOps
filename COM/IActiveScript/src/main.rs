use vbs::VBScript;
use jscript::JScript;

mod data;
mod vbs;
mod jscript;

fn main() -> windows::core::Result<()> {
    // Running the vbscript code
    let vbs = VBScript::new()?;
    let script_code = r#"
        MsgBox "Hello World"
    "#;

    vbs.run(script_code)?;

    let js = JScript::new()?;
    let script_code = r#"
        new ActiveXObject("WScript.Shell").Popup("Hello from JScript!");
    "#;

    js.run(script_code)?;

    Ok(())
}
