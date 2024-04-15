# Process Hypnosis 🦀

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

To execute the file, pass the EXE file that will be executed:
```sh
cargo run -- <file.exe>
```
```sh
target/release/process_hypnosis.exe
```

# References

* https://github.com/CarlosG13/Process-Hypnosis-Debugger-assisted-control-flow-hijack