# Patch ETW 🦀

<p align="left">
	<a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/made%20with-Rust-red"></a>
	<a href="#"><img src="https://img.shields.io/badge/platform-windows-blueviolet"></a>
</p>

- [Overview](#overview)
- [Usage](#usage)

# Overview

The ETW patching technique is used to hide trace information from a running binary. This approach is distinguished by the existence of several functions responsible for writing traces, such as EtwEventWrite, EtwEventWriteFull and EtwEventWriteEx. The common element between these functions is the invocation of the EtwpEventWrite function, which shares the same memory address.  So the rust code will identify this address and subsequently apply a patch in a unified way.

# Usage 

You can run with cargo run or the compiled binary directly:
```sh
cargo run
```
```sh
target/release/patch_etw.exe
```