use uuid::Uuid;

pub fn deobfuscate_uuid(list_uuid: Vec<&str>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut desofuscated_bytes = Vec::with_capacity(list_uuid.len() * 16);

    for uuid_str in list_uuid {
        let uuid = Uuid::parse_str(uuid_str)
            .map_err(|e| format!("Failed to parse UUID '{}': {}", uuid_str, e))?;
        desofuscated_bytes.extend_from_slice(uuid.as_bytes());
    }

    Ok(desofuscated_bytes)
}

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