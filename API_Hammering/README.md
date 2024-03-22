# API Hammering 🦀

<p align="left">
	<a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/made%20with-Rust-red"></a>
	<a href="#"><img src="https://img.shields.io/badge/platform-windows-blueviolet"></a>
</p>

- [Overview](#overview)
- [Usage](#usage)
- [References](#references)

# Overview
API Hammering consists of a large number of useless Windows API function calls, functions, loops, writing files and so on. 

In the Rust project I present two ways of carrying out this type of action, through file writing and interactions between loops.

With this technique we can escape sandbox analyses by slowly executing our malware.

# Usage 

You can run with cargo run or the compiled binary directly:
```sh
cargo run
```
```sh
target/release/api_hammering.exe
```

# References
https://github.com/rad9800/BloatedHammer

https://unit42.paloaltonetworks.com/api-hammering-malware-families

https://github.com/chvancooten/maldev-for-dummies