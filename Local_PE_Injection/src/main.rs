use clap::Parser;
use pe::PE;
use windows::core::Result;

mod pe;

/// Command-line arguments structure.
#[derive(Parser, Debug)]
#[clap(name = "pe_injection", author = "joaoviictorti", long_about = None)]
pub struct Args {
    #[clap(short, long, required = false, help = "Insert args", allow_hyphen_values = true)]
    pub arg: Option<String>,

    #[clap(short, long, required = true, help = "Insert EXE / DLL")]
    pub pe: String,

    #[clap(short, long, required = false, help = "Insert export function DLL")]
    pub export: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let path = std::path::Path::new(&args.pe);

    // Determine if the file is an EXE or DLL, and set arguments accordingly
    let (param, export) = match path.extension().and_then(|ext| ext.to_str()) {
        Some("exe") => (args.arg.clone().unwrap_or_default(), String::new()),
        Some("dll") => (String::new(), args.export.clone().unwrap_or_default()),
        _ => { 
            eprintln!("The supplied file does not have a valid extension (.exe or .dll)");
            return Ok(())
        }
    };

    // Read the PE file into memory
    let buffer = std::fs::read(&args.pe)?;

    // Initialize PE structure from buffer
    let pe = PE::new(buffer, param, export)?;

    // Load and execute the executable or DLL
    pe.run()
}
