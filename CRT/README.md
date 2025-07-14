# Remove CRT 🦀

<p align="left">
	<a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/made%20with-Rust-red"></a>
	<a href="#"><img src="https://img.shields.io/badge/platform-windows-blueviolet"></a>
</p>

- [Overview](#overview)
- [Usage](#usage)

# Overview

This project is a practical demonstration of how to build compact and efficient applications in Rust without using the C Runtime Library (CRT). The goal is to create smaller, more self-sufficient binaries, focussing on performance and security. I demonstrated the basic execution of a local payload on Windows, without using the CRT library, and presented the fundamental structure of Rust code to make it easier to use.

We chose not to use the CRT for several reasons:

* Binary Size: By removing the CRT, we significantly reduce the size of the final binary, making it easier to distribute and reducing the memory footprint.
* Detection Evasion: CRT-free applications are less likely to be detected by security solutions, making them ideal for situations that require high discretion.
* Total Control: By avoiding CRT, we gain total control over all functions and routines, allowing for specific optimisations and better resource management.

# Usage 

You can run with cargo run or the compiled binary directly:
```sh
cargo run
```
```sh
target/release/no_crt.exe
```