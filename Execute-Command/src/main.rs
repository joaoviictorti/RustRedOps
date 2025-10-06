use std::process::Command;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(target_os = "windows")] 
    {
        let command = Command::new("powershell")
            .arg("-c")
            .arg("whoami")
            .output()?;
        
        println!("{}", String::from_utf8_lossy(&command.stdout));

        Command::new("calc.exe").spawn()?;
    }
    
    #[cfg(target_os = "linux")] 
    {
        let command = Command::new("/bin/bash")
            .arg("-c")
            .arg("id")
            .output()?;

        println!("{}", String::from_utf8_lossy(&command.stdout));
    }

    Ok(())
}

