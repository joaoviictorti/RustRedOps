# minidump-rs 🦀

<p align="left">
	<a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/made%20with-Rust-red"></a>
	<a href="#"><img src="https://img.shields.io/badge/platform-windows-blueviolet"></a>
</p>

- [Overview](#overview)
- [Running](#running)
- [Usage](#usage)

# Overview

This repository presents code written in Rust designed to dump the lsass.exe process

# Running

Dumping the lsass process

![poc](img/dump.png)

# Usage

You can run with cargo run or the compiled binary directly:
```sh
cargo run
```
```sh
target/release/minidump-rs.exe
```