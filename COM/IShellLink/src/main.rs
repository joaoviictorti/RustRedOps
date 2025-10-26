use std::ptr::null_mut;
use windows::{
    core::{w, Interface},
    Win32::{
        Foundation::HWND,
        System::Com::{
            CoCreateInstance, CoInitializeEx, CoUninitialize, 
            CLSCTX_ALL, COINIT_APARTMENTTHREADED, IPersistFile,
        },
        UI::{
            Shell::{IShellLinkW, ShellLink},
            WindowsAndMessaging::SW_SHOWMAXIMIZED,
        },
    },
};

fn main() -> windows::core::Result<()> {
    unsafe {
       // Initialize COM in multithreaded mode
        CoInitializeEx(None, COINIT_APARTMENTTHREADED).ok()?;

        let shell: IShellLinkW = CoCreateInstance(&ShellLink, None, CLSCTX_ALL)?;

        // Executable path
        shell.SetPath(w!("C:\\Windows\\System32\\cmd.exe"))?;

        // Shortcut arguments
        shell.SetArguments(w!("/k echo Hello from shortcut"))?;

        // Shortcut Description
        shell.SetDescription(w!("Custom Shortcut"))?;

        // Working directory
        shell.SetWorkingDirectory(w!("C:\\Windows\\System32"))?;

        // Shortcut icon (index 0)
        shell.SetIconLocation(w!("C:\\Windows\\System32\\shell32.dll"), 0)?;

        // Keyboard shortcut (Ctrl + Shift + A = 0x0341)
        shell.SetHotkey(0x0341)?;

        // Maximized window
        shell.SetShowCmd(SW_SHOWMAXIMIZED)?;

        // Resolves the target (optional, forces path validation)
        shell.Resolve(HWND(null_mut()), 0)?;

        // Save shortcut as .lnk file
        let persist: IPersistFile = shell.cast()?;
        persist.Save(w!("C:\\Users\\Example\\Desktop\\test.lnk"), true)?;

        // Cleanup COM after operations complete
        CoUninitialize();
    }

    println!("[+] Shortcut created successfully! ✅");
    Ok(())
}
