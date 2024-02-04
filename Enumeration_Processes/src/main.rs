use sysinfo::{ProcessExt, System, SystemExt};

fn main() {
    let mut system = System::new_all();
    system.refresh_all();

    for (pid, process) in system.processes() {
        println!("Process: {} | PID: {}", process.name(), pid);
    }
}
