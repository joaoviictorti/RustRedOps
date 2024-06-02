#[derive(clap::Parser)]
#[clap(name="obfuscation", author="joaojj)", version="1.0", about="Obfuscation Shellcode", long_about = None)]
pub struct Args {
    #[clap(short, long, required = true, help = "Insert file shellcode")]
    pub file: String,

    #[clap(short, long, required = true, help = "Insert the type obfuscation")]
    pub technique: Obfuscation,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Obfuscation {
    IPV4,
    IPV6,
    MAC,
    UUID,
    WORDS
}
