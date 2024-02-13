use std::net::Ipv4Addr;

pub fn deobfuscate_ipv4(list_ips: Vec<&str>) -> Result<Vec<u8>, ()> {
    let mut deobfuscated_ips: Vec<u8> = Vec::with_capacity(list_ips.len() * 4);

    for ip in list_ips {
        match ip.parse::<Ipv4Addr>() {
            Ok(ip_addr) => {
                deobfuscated_ips.extend_from_slice(&ip_addr.octets());
            }
            Err(_) => {
                return Err(());
            }
        }
    }
    Ok(deobfuscated_ips)
}

pub fn obfuscate_ipv4(shellcode: &mut Vec<u8>) {
    if shellcode.len() % 4 != 0 {
        while shellcode.len() % 4 != 0 {
            shellcode.push(0);
        }
    }
    println!("let shellcode = vec![");
    for chunk in shellcode.chunks(4) {
        let ip = format!("{}.{}.{}.{}", chunk[0], chunk[1], chunk[2], chunk[3]);
        print!("{:?},\n", ip);
    }
    println!("];\n")
}
