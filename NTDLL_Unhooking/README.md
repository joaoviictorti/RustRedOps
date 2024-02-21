# NTDLL Unhooking (Suspend Process) 🦀

<p align="left">
	<a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/made%20with-Rust-red"></a>
	<a href="#"><img src="https://img.shields.io/badge/platform-windows-blueviolet"></a>
</p>

- [Overview](#overview)
- [Usage](#usage)

# Overview
The aim of NTDLL Unhooking is to restore the original functions of the ntdll.dll library to their initial states, removing any hooks that may have been inserted by security tools.

# Usage 

You can run with cargo run or the compiled binary directly:
```sh
cargo run
```
```sh
target/release/ntdll_unhooking.exe
```