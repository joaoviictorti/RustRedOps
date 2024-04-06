use clap::Parser;

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