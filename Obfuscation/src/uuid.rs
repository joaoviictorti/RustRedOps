use uuid::Uuid;

/// Deobfuscates a list of UUID strings into the original shellcode bytes.
///
/// # Arguments
///
/// * `list_uuid` - A vector of strings, where each string represents an obfuscated UUID (e.g., `"550e8400-e29b-41d4-a716-446655440000"`).
///
/// # Returns
///
/// * `Ok(Vec<u8>)` - A vector of bytes representing the deobfuscated shellcode.
/// * `Err(Box<dyn std::error::Error>)` - If any UUID string fails to parse correctly.
pub fn deobfuscate_uuid(list_uuid: Vec<&str>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut desofuscated_bytes = Vec::with_capacity(list_uuid.len() * 16);

    for uuid_str in list_uuid {
        let uuid = Uuid::parse_str(uuid_str)
            .map_err(|e| format!("Failed to parse UUID '{}': {}", uuid_str, e))?;
        desofuscated_bytes.extend_from_slice(uuid.as_bytes());
    }

    Ok(desofuscated_bytes)
}

/// Obfuscates a shellcode buffer into a list of UUIDs for encoding.
///
/// Each group of 16 bytes becomes a single UUID.
///
/// # Arguments
///
/// * `shellcode` - A mutable reference to a vector of bytes containing the shellcode.
///
/// # Output
///
/// Prints the list of UUIDs to the console, formatted for easy copy-paste.
pub fn obfuscate_uuid(shellcode: &mut Vec<u8>) {
    println!("let shellcode = vec![");
    let uuids = shellcode
        .chunks(16)
        .map(|chunk| {
            let mut array = [0u8; 16];
            for (i, &byte) in chunk.iter().enumerate() {
                array[i] = byte;
            }
            Uuid::from_bytes(array.into())
        });

    for uuid in uuids {
        println!("\"{}\",", uuid);
    }
    println!("];");
}