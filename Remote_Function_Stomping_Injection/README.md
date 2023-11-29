# Remote Function Stomping Injection 🦀

<p align="left">
	<a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/made%20with-Rust-red"></a>
	<a href="#"><img src="https://img.shields.io/badge/platform-windows-blueviolet"></a>
</p>

- [Overview](#overview)
- [Usage](#usage)
- [Other](#other)

# Overview

Remote Function Stomping Injection involves temporarily replacing the implementation of a system function with a malicious version as its shellcode

# Usage 

You can run with cargo run or the compiled binary directly:
```sh
cargo run
```
```sh
target/release/rfs_injection.exe
```

# Other
- To carry out this attack you have to ensure that the target process has the DLL loaded in order for the injection to be successful.