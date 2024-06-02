# Obfuscation Shellcode 🦀

<p align="left">
	<a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/made%20with-Rust-red"></a>
	<a href="#"><img src="https://img.shields.io/badge/platform-windows-blueviolet"></a>
</p>

- [Overview](#overview)
- [Usage](#usage)

# Overview
Shellcode obfuscation is a fundamental technique in the development of malware, with the aim of avoiding detection by antivirus software and security analysis systems.  This project uses an approach to shellcode obfuscation, incorporating IPv4 and IPv6 addressing formats, MAC addresses UUIDs and WORDS.

Each directory contains the method needed to perform de-obfuscation for IPV4, IPV6, MAC, UUID and WORDS techniques.
# Usage 

You can run with cargo run or the compiled binary directly:
```sh
cargo run -- -f shell.bin -t ipv4
```
```sh
target/release/obfuscation.exe -f shell.bin -t ipv4
```
If you have any doubts, you can check the help using --help:
```sh
target/release/obfuscation.exe --help
```