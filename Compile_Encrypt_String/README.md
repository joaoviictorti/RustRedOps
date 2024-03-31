# Encrypting strings at compile time 🦀

<p align="left">
	<a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/made%20with-Rust-red"></a>
	<a href="#"><img src="https://img.shields.io/badge/platform-windows-blueviolet"></a>
</p>

- [Overview](#overview)
- [Usage](#usage)
- [References](#references)

# Overview

This project focuses on demonstrating how you can encrypt strings at compile time and decrypt them at runtime.

The project is just a starting point for you to improve the project and implement cryptography that you find interesting. In this example, I'm focusing on XOR, but use it however you prefer.

# Usage 

In the "usage_lib" directory I demonstrate how to use the code, but you can implement this line in your Cargo.toml:
```sh
encrypt_string = { path = "../encrypt_string" }
```

Then call it in your code:
```rust
use encrypt_string::encrypt_string;

fn main() {
    let nome = encrypt_string!("I'm encrypted!");
    println!("{}", nome);
}
```

# References

* https://github.com/anvie/litcrypt.rs