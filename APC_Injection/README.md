# APC Injection 🦀

<p align="left">
	<a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/made%20with-Rust-red"></a>
	<a href="#"><img src="https://img.shields.io/badge/platform-windows-blueviolet"></a>
</p>

- [Overview](#overview)
- [Usage](#usage)

# Overview
APC injection is an attack technique that exploits Windows' ability to schedule asynchronous procedure calls (APCs). APCs are used to execute code in a process in a specific context, usually as part of system operations. Attackers can exploit this functionality by injecting malicious code into a target process to execute their own instructions. 

# Usage 

You can run with cargo run or the compiled binary directly:
```sh
cargo run
```
```sh
target/release/apc_injection.exe
```