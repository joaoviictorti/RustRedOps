# Shellcode execution via Callback 🦀

<p align="left">
	<a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/made%20with-Rust-red"></a>
	<a href="#"><img src="https://img.shields.io/badge/platform-windows-blueviolet"></a>
</p>

- [Overview](#overview)
- [Usage](#usage)

# Overview
This repository focuses on executing shellcode via callback calls. In the code, we used the `EnumCalendarInfo` function, however, it is important to note that there are several other functions that can also be used for this purpose. 

# Usage 

You can run with cargo run or the compiled binary directly:
```sh
cargo run
```
```sh
target/release/shellcode_callback.exe
```