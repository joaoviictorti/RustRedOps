# Process Argument Spoofing 🦀

<p align="left">
	<a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/made%20with-Rust-red"></a>
	<a href="#"><img src="https://img.shields.io/badge/platform-windows-blueviolet"></a>
</p>

- [Overview](#overview)
- [Running](#running)
- [Usage](#usage)


# Overview
The "Process Argument Spoofing" technique involves modifying or masking the arguments passed to a process running on an operating system. The aim is to trick users, system administrators or security tools into misinterpreting the process, thinking it is harmless or legitimate, while in fact it may be carrying out malicious actions.

# Running

Showing changes to arguments in Procmon and Process Hacker.

![poc](img/poc.png)

# Usage 

You can run with cargo run or the compiled binary directly:
```sh
cargo run
```
```sh
target/release/args_spoofing-rs.exe
```