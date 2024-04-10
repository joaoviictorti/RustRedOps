# Process Herpaderping 🦀

<p align="left">
	<a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/made%20with-Rust-red"></a>
	<a href="#"><img src="https://img.shields.io/badge/platform-windows-blueviolet"></a>
</p>

- [Overview](#overview)
- [Usage](#usage)
- [References](#references)

# Overview

Process Herpaderping is a technique that aims to hide the true intentions of a process by altering the disk contents after the process image has already been loaded into memory. This procedure causes an unusual reaction in both security systems and the operating system itself.

# Usage 

To execute the file, pass the EXE file that will be executed:
```sh
cargo run -- <file.exe> <args> <path>
```
```sh
target/release/process_herpaderping.exe <file.exe> <args> <path>
```

Real use:
```sh
target/release/process_herpaderping.exe mimikatz.exe "coffee localtime exit" C:\Windows\System32\OneDriveSetup.exe
```

# References

* https://github.com/jxy-s/herpaderping