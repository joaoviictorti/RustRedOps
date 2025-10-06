use herpaderping::Herpaderping;

mod herpaderping;
mod wrappers;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: process_herpaderping.exe <file.exe> <args> <file-path>");
        return Ok(())
    }

    let herpaderping = Herpaderping::new(&args[1], &args[2], &args[3])?;
    herpaderping.run()
}
