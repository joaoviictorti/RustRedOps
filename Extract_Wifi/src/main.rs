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
        let mut result = 0;
        result = WlanOpenHandle(
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

        for interface in interfaces_list {
            let mut wlan_profiles_ptr = null_mut();
            result = WlanGetProfileList(
                wlan_handle,
                &interface.InterfaceGuid,
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
                let profile_info = String::from_utf16_lossy(&profile.strProfileName)
                    .trim_matches('\0')
                    .to_string();
                let mut xml_data = PWSTR::null();
                let mut flag = WLAN_PROFILE_GET_PLAINTEXT_KEY;
                result = WlanGetProfile(
                    wlan_handle,
                    &interface.InterfaceGuid,
                    PCWSTR(HSTRING::from(profile_info.clone()).as_ptr()),
                    None,
                    &mut xml_data,
                    Some(&mut flag),
                    None,
                );

                if result != ERROR_SUCCESS.0 {
                    WlanCloseHandle(wlan_handle, None);
                    panic!("WlanGetProfile Failed With Error: {}", result);
                }

                let mut len = 0;
                while *xml_data.0.offset(len) != 0 {
                    len += 1;
                }
                let xml_slice = std::slice::from_raw_parts(xml_data.0, len as usize);
                let xml = String::from_utf16_lossy(xml_slice);
                let mut reader = Reader::from_str(&xml);
                reader.trim_text(true);
                let mut in_shared_key = false;
                let mut key_material = String::new();

                loop {
                    match reader.read_event() {
                        Ok(Event::Start(ref e)) => {
                            if e.name() == quick_xml::name::QName(b"keyMaterial") {
                                in_shared_key = true;
                            }
                        }
                        Ok(Event::Text(ref e)) if in_shared_key => {
                            key_material = e.escape_ascii().to_string();
                            in_shared_key = false;
                        }
                        Ok(Event::Eof) => break,
                        Err(e) => panic!("Error parsing the XML: {:?}", e),
                        _ => (),
                    }
                }

                if !key_material.is_empty() {
                    println!("WIFI: {} | PASSWORD: {}", profile_info, key_material);
                } else {
                    println!("WIFI {} | PASSWORD NOT FOUND.", profile_info);
                }
            }
        }
    }
}
