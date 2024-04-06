# Module Overloading 🦀

<p align="left">
	<a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/made%20with-Rust-red"></a>
	<a href="#"><img src="https://img.shields.io/badge/platform-windows-blueviolet"></a>
</p>

- [Overview](#overview)
- [Usage](#usage)
- [References](#references)

# Overview

Module overloading is a method of swapping the contents of a DLL mapped to our process using the NtCreateSection and NtMapViewOfSection APIs, then replacing its contents with an EXE/DLL file and subsequently executing its entry point.

# Usage 

You can pass a DLL file or an EXE to the binary:
```sh
cargo run -- -f <file.exe / file.dll> --dll "<path-dll>"
```
```sh
target/release/module_overloading.exe -f <file.exe / file.dll> --dll "<path-dll>"
```

You can pass arguments to your file, in this example I'm using mimikatz.exe
```sh
cargo run -- -f mimikatz.exe --dll "C:\Windows\System32\user32.dll" --args "coffee exit"
```

# References

* https://github.com/hasherezade/module_overloading