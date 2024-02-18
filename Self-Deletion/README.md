# Self Deletion 🦀

<p align="left">
	<a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/made%20with-Rust-red"></a>
	<a href="#"><img src="https://img.shields.io/badge/platform-windows-blueviolet"></a>
</p>

- [Overview](#overview)
- [Usage](#usage)

# Overview
This project focuses on the self-deletion technique, which allows the binary itself to be deleted during execution. This functionality is particularly useful in scenarios where it is detected that the binary is under analysis, either through debugging or execution in a Virtual Machine (VM)

# Usage 
You can run with cargo run or the compiled binary directly:
```sh
cargo run
```
```sh
target/release/self_deletion.exe
```