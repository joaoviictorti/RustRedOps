use clap::{Parser, ValueEnum};
use ipv4::{deobfuscate_ipv4, obfuscate_ipv4};
use ipv6::{deobfuscate_ipv6, obfuscate_ipv6};
use mac::{deobfuscate_mac, obfuscate_mac};
use uuid::{deobfuscate_uuid, obfuscate_uuid};
use words::{deobfuscate_words, obfuscate_words};

mod ipv4;
mod ipv6;
mod mac;
mod uuid;
mod words;

#[derive(ValueEnum, Clone, Debug)]
pub enum Obfuscation {
    IPV4,
    IPV6,
    MAC,
    UUID,
    WORDS,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum Action {
    Obfuscate,
    Deobfuscate,
}

#[derive(Parser)]
#[clap(
    name = "obfuscation",
    author = "joaojj",
    about = "Obfuscation Shellcode Tool",
    long_about = None
)]
pub struct Args {
    #[clap(short, long, required = true, help = "Insert file shellcode")]
    pub file: String,

    #[clap(
        short,
        long,
        required = true,
        help = "Insert the obfuscation technique"
    )]
    pub technique: Obfuscation,

    #[clap(
        short,
        long,
        required = true,
        help = "Choose whether to obfuscate or deobfuscate"
    )]
    pub action: Action,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mut buffer = std::fs::read(&args.file)?;

    match args.action {
        Action::Obfuscate => match args.technique {
            Obfuscation::IPV4 => obfuscate_ipv4(&mut buffer),
            Obfuscation::IPV6 => obfuscate_ipv6(&mut buffer),
            Obfuscation::MAC => obfuscate_mac(&mut buffer),
            Obfuscation::UUID => obfuscate_uuid(&mut buffer),
            Obfuscation::WORDS => obfuscate_words(&mut buffer),
        },
        Action::Deobfuscate => {
            let content = String::from_utf8(buffer)?;
            let lines: Vec<&str> = content
                .lines()
                .map(|line| line.trim_matches(|c| c == '"' || c == ','))
                .filter(|line| !line.is_empty() && !line.starts_with("let shellcode") && !line.starts_with(']'))
                .collect();

            let shellcode = match args.technique {
                Obfuscation::IPV4 => deobfuscate_ipv4(lines),
                Obfuscation::IPV6 => deobfuscate_ipv6(lines),
                Obfuscation::MAC => deobfuscate_mac(lines),
                Obfuscation::UUID => deobfuscate_uuid(lines),
                Obfuscation::WORDS => deobfuscate_words(lines),
            }?;

            println!("let shellcode = vec![");
            for (i, byte) in shellcode.iter().enumerate() {
                print!("0x{:02X}, ", byte);
                if (i + 1) % 16 == 0 {
                    println!();
                }
            }

            println!("\n];");
        }
    };

    Ok(())
}
