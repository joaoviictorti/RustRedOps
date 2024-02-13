use uuid::Uuid;

pub fn deobfuscate_uuid(list_uuid: Vec<&str>) -> Result<Vec<u8>, ()> {
    let mut desofuscated_bytes = Vec::new();

    for uuid_str in list_uuid {
        match Uuid::parse_str(uuid_str) {
            Ok(uuid) => {
                desofuscated_bytes.extend_from_slice(uuid.as_bytes());
            }
            Err(_) => return Err(()),
        }
    }

    Ok(desofuscated_bytes)
}

pub fn obfuscate_uuid(shellcode: &mut Vec<u8>) {
    println!("let shellcode = vec![");
    let uuids: Vec<Uuid> = shellcode
        .chunks(16)
        .map(|chunk| {
            let mut array = [0; 16];
            for (i, &byte) in chunk.iter().enumerate() {
                array[i] = byte;
            }
            Uuid::from_bytes(array)
        })
        .collect();

    for uuid in &uuids {
        print!("\"{}\",\n", uuid);
    }

    println!("];")
}
