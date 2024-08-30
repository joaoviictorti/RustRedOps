use clap::Parser;

#[derive(Parser)]
#[clap(name="local_pe_injection", author="joaojj", long_about = None)]
pub struct Args {
    #[clap(short, long, required = false, help = "Insert args")]
    pub arg: Option<String>,

    #[clap(short, long, required = true, help = "Insert EXE / DLL")]
    pub pe: String,

    #[clap(short, long, required = false, help = "Insert export function DLL")]
    pub export: Option<String>,
}