use std::collections::HashMap;
use wmi::{COMLibrary, Variant, WMIConnection};

fn main() -> Result<(), wmi::WMIError> {
    let _com_library = COMLibrary::new()?;
    let wmi_connection = unsafe { WMIConnection::with_initialized_com(Some("root\\SecurityCenter2"))? };
    let avs: Vec<HashMap<String, Variant>> = wmi_connection.raw_query("SELECT * FROM AntiVirusProduct")?;
    for result in avs {
        println!("Infos AntivirusProduct:");
        println!("{:#?}", result);
    }

    Ok(())
}
