# Local PE Injection 🦀

<p align="left">
	<a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/made%20with-Rust-red"></a>
	<a href="#"><img src="https://img.shields.io/badge/platform-windows-blueviolet"></a>
</p>

- [Overview](#overview)
- [Usage](#usage)

# Overview

Local PE Injection is a method to execute a PE file in memory.

# Usage 

You can pass a DLL file or an EXE to the binary:
```sh
cargo run -- --pe <file.exe / file.dll> --export <function>"
```
```sh
target/release/local_pe_injection.exe --pe <file.exe / file.dll>"
```

You can pass arguments to your file, in this example I'm using mimikatz.exe
```sh
cargo run -- --pe mimikatz.exe --arg "coffee exit"
```

Using DLL:
```sh
cargo run --release -- --pe .\main.dll --export "test"
```
