use std::net::Ipv4Addr;

/// Deobfuscates a list of IPv4 address strings into the original shellcode bytes.
///
/// # Arguments
///
/// * `list_ips` - A vector of strings, where each string represents an obfuscated IPv4 address (e.g., `"192.168.0.1"`).
///
/// # Returns
///
/// * `Ok(Vec<u8>)` - A vector of bytes representing the deobfuscated shellcode.
/// * `Err(Box<dyn std::error::Error>)` - If any IP string fails to parse correctly.
pub fn deobfuscate_ipv4(list_ips: Vec<&str>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut deobfuscated_ips: Vec<u8> = Vec::with_capacity(list_ips.len() * 4);

    for ip in list_ips {
        let ip_addr = ip
            .parse::<Ipv4Addr>()
            .map_err(|e| format!("Failed to parse IP '{}': {}", ip, e))?;
        deobfuscated_ips.extend_from_slice(&ip_addr.octets());
    }

    Ok(deobfuscated_ips)
}

/// Obfuscates a shellcode buffer into a list of IPv4 addresses for encoding.
///
/// Each group of 4 bytes becomes an IP address.
///
/// # Arguments
///
/// * `shellcode` - A mutable reference to a vector of bytes containing the shellcode.
///   - If the shellcode length is not a multiple of 4, it will be **padded with zeros**.
///
/// # Output
///
/// Prints the list of IPv4 addresses to the console, formatted for easy copy-paste.
pub fn obfuscate_ipv4(shellcode: &mut Vec<u8>) {
    if shellcode.len() % 4 != 0 {
        shellcode.resize((shellcode.len() + 3) / 4 * 4, 0);
    }

    println!("let shellcode = vec![");
    for chunk in shellcode.chunks(4) {
        let ip = format!("{}.{}.{}.{}", chunk[0], chunk[1], chunk[2], chunk[3]);
        println!("{:?},", ip);
    }

    println!("];");
}
