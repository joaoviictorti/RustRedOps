use quick_xml::{events::Event, Reader};
use std::ptr::null_mut;
use windows::{
    core::{HSTRING, PCWSTR, PWSTR},
    Win32::{
        Foundation::{ERROR_SUCCESS, HANDLE},
        NetworkManagement::WiFi::{
            WlanCloseHandle, WlanEnumInterfaces, WlanGetProfile, WlanGetProfileList,
            WlanOpenHandle, WLAN_API_VERSION_2_0, WLAN_PROFILE_GET_PLAINTEXT_KEY,
        },
    },
};

fn main() {
    unsafe {
        let mut negotiate_version = 0;
        let mut wlan_handle = HANDLE::default();
        let mut result = WlanOpenHandle(
            WLAN_API_VERSION_2_0,
            None,
            &mut negotiate_version,
            &mut wlan_handle,
        );
        

        if result != ERROR_SUCCESS.0 {
            panic!("WlanOpenHandle Failed With Error: {}", result);
        }

        let mut interface = null_mut();
        result = WlanEnumInterfaces(wlan_handle, None, &mut interface);
        if result != ERROR_SUCCESS.0 {
            WlanCloseHandle(wlan_handle, None);
            panic!("WlanEnumInterfaces Failed With Error: {}", result);
        }

        let interfaces_list = std::slice::from_raw_parts(
            (*interface).InterfaceInfo.as_ptr(),
            (*interface).dwNumberOfItems as usize,
        );

        for wlan_interface in interfaces_list {
            let mut wlan_profiles_ptr = null_mut();
            result = WlanGetProfileList(
                wlan_handle,
                &wlan_interface.InterfaceGuid,
                None,
                &mut wlan_profiles_ptr,
            );

            if result != ERROR_SUCCESS.0 {
                WlanCloseHandle(wlan_handle, None);
                panic!("WlanGetProfileList Failed With Error: {}", result);
            }

            let wlan_profile_list = std::slice::from_raw_parts(
                (*wlan_profiles_ptr).ProfileInfo.as_ptr(),
                (*wlan_profiles_ptr).dwNumberOfItems as usize,
            );

            for profile in wlan_profile_list {
                let profile_name = String::from_utf16_lossy(&profile.strProfileName)
                    .trim_matches('\0')
                    .to_string();

                let mut xml_data = PWSTR::null();
                let mut flag = WLAN_PROFILE_GET_PLAINTEXT_KEY;
                result = WlanGetProfile(
                    wlan_handle,
                    &wlan_interface.InterfaceGuid,
                    PCWSTR(HSTRING::from(&profile_name).as_ptr()),
                    None,
                    &mut xml_data,
                    Some(&mut flag),
                    None,
                );

                if result != ERROR_SUCCESS.0 {
                    WlanCloseHandle(wlan_handle, None);
                    panic!("WlanGetProfile Failed With Error: {}", result);
                }

                let xml = pwstr_to_string(xml_data);
                let key_material = extract_key_material(&xml);

                if !key_material.is_empty() {
                    println!("WIFI: {} | PASSWORD: {}", profile_name, key_material);
                } else {
                    println!("WIFI: {} | PASSWORD NOT FOUND.", profile_name);
                }
            }
        }
    }
}

/// Converts a `PWSTR` (UTF-16 string pointer) to a `String`.
fn pwstr_to_string(pwstr: PWSTR) -> String {
    unsafe {
        let mut len = 0;
        while *pwstr.0.offset(len) != 0 {
            len += 1;
        }

        let utf16_slice = std::slice::from_raw_parts(pwstr.0, len as usize);
        String::from_utf16_lossy(utf16_slice)
    }
}

/// Extracts the value of `keyMaterial` from an XML.
fn extract_key_material(xml: &str) -> String {
    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);

    let mut in_key_material = false;
    let mut key_material = String::new();

    loop {
        match reader.read_event() {
            Ok(Event::Start(ref e)) if e.name() == quick_xml::name::QName(b"keyMaterial") => {
                in_key_material = true;
            }
            Ok(Event::Text(ref e)) if in_key_material => {
                key_material = e.escape_ascii().to_string();
                in_key_material = false;
            }
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error processing XML: {:?}", e),
            _ => (),
        }
    }

    key_material
}