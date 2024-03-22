use std::fs::{remove_file, File};
use std::io::{Read, Write};
use rand::{thread_rng, Rng};
use std::env;

// 1 Method
fn api_hammering(num: usize) -> std::io::Result<()> {
    let dir = env::temp_dir();
    let file_path = dir.as_path().join("file.tmp");
    let buffer_size = 0xFFFFF;

    for _ in 0..num {
        // Creates the file and writes random data
        let mut file = File::create(&file_path)?;
        let mut rng = thread_rng();
        let data: Vec<u8> = (0..buffer_size).map(|_| rng.gen()).collect();
        file.write_all(&data)?;

        // Read written data
        let mut file = File::open(&file_path)?;
        let mut buffer = vec![0; buffer_size];
        file.read_exact(&mut buffer)?;
    }

    remove_file(file_path)?;

    Ok(())
}

// 2 Method
// https://github.com/chvancooten/maldev-for-dummies/blob/main/Exercises/Exercise%203%20-%20Basic%20AV%20Evasion/solutions/rust/src/basic_av_evasion.rs#L29
#[no_mangle]
#[inline(never)]
fn calc_primes(iterations: usize) {
    let mut prime = 2;
    let mut i = 0;
    while i < iterations {
        if (2..prime).all(|j| prime % j != 0) {
            i += 1;
        }
        prime += 1;
    }
}


fn main() {
    println!("[+] First method triggered");
    let number = 2000; // Defines the number of times the API will be "hammered"
    match api_hammering(number) {
        Ok(_) => println!("[+] API Hammering successfully completed!"),
        Err(e) => println!("[!] Error during API hammering: {}", e),
    }

    println!("[+] Second method triggered");
    calc_primes(number)
}