use windows_core::{BSTR, GUID, VARIANT};
use windows::Win32::{
    System::Com::{CoCreateInstance, CoInitializeEx, CLSCTX_ALL, COINIT_MULTITHREADED},
    UI::Shell::IShellDispatch2,
};

/// CLSID for `Shell.Application`, which provides access to `IShellDispatch2`
const CLSID_SHELL: GUID = GUID::from_u128(0x13709620_C279_11CE_A49E_444553540000);

fn main() -> windows::core::Result<()> {
    unsafe {
        // Initialize COM in multithreaded apartment
        CoInitializeEx(None, COINIT_MULTITHREADED).ok()?;

        // Create Shell.Application COM object
        let shell: IShellDispatch2 = CoCreateInstance(&CLSID_SHELL, None, CLSCTX_ALL)?;

        // Use ShellExecute to launch PowerShell with a command
        shell.ShellExecute(
            &BSTR::from("powershell.exe"),
            &VARIANT::from("-NoExit -Command Write-Host 'Hello from Rust!'; Start-Sleep -Seconds 9999"),
            &VARIANT::default(),
            &VARIANT::default(),
            &VARIANT::from(1), // SW_SHOWNORMAL
        )?;
    }

    Ok(())
}
