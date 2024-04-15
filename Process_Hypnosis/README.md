# Process Hypnosis 🦀

<p align="left">
	<a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/made%20with-Rust-red"></a>
	<a href="#"><img src="https://img.shields.io/badge/platform-windows-blueviolet"></a>
</p>

- [Overview](#overview)
- [Usage](#usage)
- [References](#references)

# Overview

The idea behind Process Hypnosis is to develop a malicious artifact that behaves like a debugger. As a result, we gain the ability to control the execution flow of a program that is being debugged and obtain relevant information from it, such as: creation of new threads, loaded modules, exceptions and much more.

# Usage 

Run the program directly or compile and use the exe afterwards:
```sh
cargo run
```
```sh
target/release/process_hypnosis.exe
```

# References

* https://github.com/CarlosG13/Process-Hypnosis-Debugger-assisted-control-flow-hijack