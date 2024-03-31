# Module Stomping 🦀

<p align="left">
	<a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/made%20with-Rust-red"></a>
	<a href="#"><img src="https://img.shields.io/badge/platform-windows-blueviolet"></a>
</p>

- [Overview](#overview)
- [Usage](#usage)
- [References](#references)

# Overview

The Module Stomping technique is a way of mapping or loading a DLL in your process or in another process and then retrieving the AddressOfEntryPoint address to replace it with its shellcode and execute it.

# Usage 

You can run with cargo run or the compiled binary directly:
```sh
cargo run
```
```sh
target/release/module_stomping.exe
```

# References

* https://github.com/WithSecureLabs/ModuleStomping
* https://www.ired.team/offensive-security/code-injection-process-injection/modulestomping-dll-hollowing-shellcode-injection