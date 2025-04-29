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
