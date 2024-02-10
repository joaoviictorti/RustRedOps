# Block DLL Policy 🦀

<p align="left">
	<a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/made%20with-Rust-red"></a>
	<a href="#"><img src="https://img.shields.io/badge/platform-windows-blueviolet"></a>
</p>

- [Overview](#overview)
- [Usage](#usage)

# Overview
The "Block DLL Policy" technique is an effective strategy for preventing non-Microsoft-signed DLLs from being loaded into system processes. This policy can be applied both when creating new processes and implemented in our local process.

# Usage 

You can run with cargo run or the compiled binary directly:
```sh
cargo run
```
```sh
target/release/block_dll_policy.exe
```