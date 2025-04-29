use std::net::Ipv6Addr;

/// Deobfuscates a list of IPv6 address strings into the original shellcode bytes.
///
/// # Arguments
///
/// * `list_ips` - A vector of strings, where each string represents an obfuscated IPv6 address (e.g., `"2001:0db8::1"`).
///
/// # Returns
///
/// * `Ok(Vec<u8>)` - A vector of bytes representing the deobfuscated shellcode.
/// * `Err(Box<dyn std::error::Error>)` - If any IPv6 string fails to parse correctly.
pub fn deobfuscate_ipv6(list_ips: Vec<&str>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut deobfuscated_ips: Vec<u8> = Vec::with_capacity(list_ips.len() * 16);

    for ip in list_ips {
        let ip_addr = ip
            .parse::<Ipv6Addr>()
            .map_err(|e| format!("Failed to parse IPv6 '{}': {}", ip, e))?;

        for segment in ip_addr.segments() {
            deobfuscated_ips.extend_from_slice(&segment.to_be_bytes());
        }
    }

    Ok(deobfuscated_ips)
}

/// Obfuscates a shellcode buffer into a list of IPv6 addresses for encoding.
///
/// Each group of 16 bytes becomes a single IPv6 address.
///
/// # Arguments
///
/// * `shellcode` - A mutable reference to a vector of bytes containing the shellcode.
///   - If the shellcode length is not a multiple of 16, it will be **padded with zeros**.
///
/// # Output
///
/// Prints the list of IPv6 addresses to the console, formatted for easy copy-paste.
pub fn obfuscate_ipv6(shellcode: &mut Vec<u8>) {
    if shellcode.len() % 16 != 0 {
        shellcode.resize((shellcode.len() + 15) / 16 * 16, 0);
    }

    println!("let shellcode = vec![");
    for chunk in shellcode.chunks(16) {
        let ip = format!(
            "{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:\
             {:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}",
            chunk[0],
            chunk[1],
            chunk[2],
            chunk[3],
            chunk[4],
            chunk[5],
            chunk[6],
            chunk[7],
            chunk[8],
            chunk[9],
            chunk[10],
            chunk[11],
            chunk[12],
            chunk[13],
            chunk[14],
            chunk[15]
        );

        println!("{:?},", ip);
    }

    println!("];");
}
