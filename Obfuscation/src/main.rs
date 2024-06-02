mod ipv4;
mod ipv6;
mod mac;
mod utils;
mod uuid;
mod words;

use clap::Parser;
use ipv4::obfuscate_ipv4;
use ipv6::obfuscate_ipv6;
use mac::obfuscate_mac;
use uuid::obfuscate_uuid;
use words::obfuscate_words;
use std::{fs::File, io::Read};
use utils::{Args, Obfuscation};


fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let file = args.file;
    let technique = args.technique;
    let mut shellcode = File::open(file)?;
    let mut buffer: Vec<u8> = Vec::new();
    shellcode.read_to_end(&mut buffer)?;

    match technique {
        Obfuscation::IPV4 => {
            obfuscate_ipv4(&mut buffer);
        }
        Obfuscation::IPV6 => {
            obfuscate_ipv6(&mut buffer);
        },
        Obfuscation::MAC => {
            obfuscate_mac(&mut buffer);
        }
        Obfuscation::UUID => {
            obfuscate_uuid(&mut buffer);
        },
        Obfuscation::WORDS => {
            obfuscate_words(&mut buffer);
        }
    };

    Ok(())
}
