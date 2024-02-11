use std::process::Command;

fn main() {
    #[cfg(target_os = "windows")] {
    let command = Command::new("powershell")
        .arg("-c")
        .arg("whoami")
        .output()
        .unwrap();
    println!("{}", String::from_utf8_lossy(&command.stdout));
    let _ = Command::new("calc.exe").spawn();
    }
    
    #[cfg(target_os = "linux")] {
    let command = Command::new("/bin/bash")
        .arg("-c")
        .arg("id")
        .output()
        .unwrap();
    println!("{}", String::from_utf8_lossy(&command.stdout));
    }
}

