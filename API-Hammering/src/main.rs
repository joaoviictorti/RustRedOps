use std::fs::{remove_file, File};
use std::io::{self, Read, Write};
use rand::{thread_rng, Rng};

/// This function simulates "API hammering" by rapidly creating a file in the system's temp directory,
/// writing a large buffer of random bytes to it, and reading it back. This is done repeatedly
/// to increase system noise or potentially evade sandbox analysis.
///
/// # Parameters
///
/// * `num` - The number of I/O iterations to perform.
///
/// # Returns
///
/// * `Ok(())` on success.
/// * `Err(io::Error)` if any file operation fails.
fn api_hammering(num: usize) -> io::Result<()> {
    let dir = std::env::temp_dir();
    let path = dir.as_path().join("file.tmp");
    let size = 0xFFFFF;

    for _ in 0..num {
        // Creates the file and writes random data
        let mut file = File::create(&path)?;
        let mut rng = thread_rng();
        let data: Vec<u8> = (0..size).map(|_| rng.gen()).collect();
        file.write_all(&data)?;

        // Read written data
        let mut file = File::open(&path)?;
        let mut buffer = vec![0; size];
        file.read_exact(&mut buffer)?;
    }

    remove_file(path)?;

    Ok(())
}

/// Calculates a sequence of prime numbers using brute-force method.
///
/// This function simulates heavy CPU-bound computation by iterating through integers
/// and checking primality using division tests. Useful for stress testing or generating
/// delays in execution.
/// 
/// # Parameters
///
/// * `iterations` - Number of prime numbers to find.
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
    api_hammering(2000).expect("Error during API hammering");

    println!("[+] Second method triggered");
    calc_primes(2000)
}