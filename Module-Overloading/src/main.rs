#![allow(unused_assignments)]

use clap::Parser;
use module::Module;
use windows::core::Result;

mod module;

/// Command-line arguments structure.
#[derive(Parser)]
#[clap(name="module_overloading", author="joaoviictorti)", about="Module Overloading", long_about = None)]
pub struct Args {
    #[clap(short, long, required = true, help = "Insert an EXE or DLL file ")]
    pub file: String,

    #[clap(short, long, help = "Insert the DLL to be mapped")]
    pub dll: String,

    #[clap(short, long, help = "Insert the arguments for the target file to be executed")]
    pub args: Option<String>
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Read the target PE file (EXE or DLL) into memory
    let buffer = std::fs::read(&args.file)?;
    
    // Create a Module instance with the loaded buffer, arguments, and target DLL
    let module = Module::new(
        buffer,
        args.args.unwrap_or_else(|| "".to_string()),
        args.dll,
    )?;

    // Execute the module
    module.run()
}
