# DLL Injection 🦀

<p align="left">
	<a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/made%20with-Rust-red"></a>
	<a href="#"><img src="https://img.shields.io/badge/platform-windows-blueviolet"></a>
</p>

- [Overview](#overview)
- [Usage](#usage)
- [Other](#other)

# Overview

This repository features code written in Rust intended to exploit DLL injection into a remote process.

# Usage

You can run with cargo run or the compiled binary directly:
```sh
cargo run -- <pid> <full-path-of-the-DLL>
```
```sh
target/release/dllinjection_rs.exe <pid> <full-path-of-the-DLL>
```

# Other
- An example of a DLL in the /dll folder if you want to perform the tests quickly, before using, compile the dll as shown previously