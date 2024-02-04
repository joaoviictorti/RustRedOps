# IAT OBFUSCATION 🦀

<p align="left">
        <a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/made%20with-Rust-red"></a>
        <a href="#"><img src="https://img.shields.io/badge/platform-windows-blueviolet"></a>
</p>

- [Overview](#overview)
- [Usage](#usage)

# Overview

This project presents an IAT obfuscation technique, which is a way of retrieving addresses, ordinals and API names in DLLs, such as ntdll.dll and kernel32.dll.

Often, we can't use GetModuleHandle and GetProcAddress directly because of detection by security solutions. So this technique allows you to retrieve information without having to use them.

# Usage

You can run with cargo run or the compiled binary directly:
```sh
cargo run
```
```sh
target/release/iat_obfuscation.exe
```