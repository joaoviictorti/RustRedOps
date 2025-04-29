/// Deobfuscates a list of MAC address strings into the original shellcode bytes.
///
/// # Arguments
///
/// * `mac_addresses` - A vector of strings, where each string represents a MAC address (e.g., `"AA:BB:CC:DD:EE:FF"`).
///
/// # Returns
///
/// * `Ok(Vec<u8>)` - A vector of bytes representing the deobfuscated shellcode.
/// * `Err(Box<dyn std::error::Error>)` - If any MAC segment fails to parse correctly.
pub fn deobfuscate_mac(mac_addresses: Vec<&str>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut original_ints = Vec::with_capacity(mac_addresses.len() * 6);

    for mac in mac_addresses {
        for byte_str in mac.split(':') {
            let byte = u8::from_str_radix(byte_str, 16)
                .map_err(|e| format!("Failed to parse MAC segment '{}': {}", byte_str, e))?;
            original_ints.push(byte);
        }
    }

    Ok(original_ints)
}


/// Obfuscates a shellcode buffer into a list of MAC addresses for encoding.
///
/// Each group of 6 bytes becomes a single MAC address.
///
/// # Arguments
///
/// * `shellcode` - A mutable reference to a vector of bytes containing the shellcode.
///
/// # Output
///
/// Prints the list of MAC addresses to the console, formatted for easy copy-paste.
pub fn obfuscate_mac(shellcode: &mut Vec<u8>) {
    println!("let shellcode = vec![");
    let mac_addresses = shellcode.chunks(6).map(|chunk| {
        chunk
            .iter()
            .map(|byte| format!("{:02X}", byte))
            .collect::<Vec<_>>()
            .join(":")
    });

    for mac in mac_addresses {
        println!("\"{}\",", mac);
    }
    println!("];");
}
