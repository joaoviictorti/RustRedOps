pub fn deobfuscate_mac(mac_addresses: Vec<&str>) -> Result<Vec<u8>, ()> {
    let original_ints: Vec<u8> = mac_addresses
        .iter()
        .flat_map(|mac| {
            mac.split(':')
                .map(|byte_str| u8::from_str_radix(byte_str, 16).unwrap())
                .collect::<Vec<u8>>()
        })
        .collect();

    Ok(original_ints)
}

pub fn obfuscate_mac(shellcode: &mut Vec<u8>) {
    println!("let shellcode = vec![");

    let mac_addresses: Vec<String> = shellcode
        .chunks(6)
        .map(|chunk| {
            chunk
                .iter()
                .map(|byte| format!("{:02X}", byte))
                .collect::<Vec<String>>()
                .join(":")
        })
        .collect();

    for mac in &mac_addresses {
        print!("\"{}\",\n", mac);
    }

    println!("];")
}
