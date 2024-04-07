# Process Ghosting 🦀

<p align="left">
	<a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/made%20with-Rust-red"></a>
	<a href="#"><img src="https://img.shields.io/badge/platform-windows-blueviolet"></a>
</p>

- [Overview](#overview)
- [Usage](#usage)
- [References](#references)

# Overview

Process Ghosting is a technique that uses a temporary file with pending deletion to create a process from that content.

# Usage 

You can pass a DLL file or an EXE to the binary:
```sh
cargo run -- <file.exe>
```
```sh
target/release/process_ghosting.exe -f <file.exe>
```

# References

* https://github.com/hasherezade/process_ghosting