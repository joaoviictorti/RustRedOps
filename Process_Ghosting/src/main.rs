mod ghosting;
mod wrappers;

use ghosting::Ghosting;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 3 {
        eprintln!("Usage: Process_Ghosting.exe <exe> <args>");
        return Ok(())
    }

    let ghost = Ghosting::new(&args[1], &args[2])?;
    ghost.run()
}
