# Encrypting strings at compile time 🦀

This project focuses on demonstrating how you can encrypt strings at compile time and decrypt them at runtime.

The project is just a starting point for you to improve the project and implement cryptography that you find interesting. In this example, I'm focusing on XOR, but use it however you prefer.

## Usage 

In the "bof" directory I demonstrate how to use the code, but you can implement this line in your Cargo.toml:
```sh
obf = { path = "../obf" }
```

Then call it in your code:
```rust
use obf::obf;

fn main() {
    let nome = obf!("I'm encrypted!");
    println!("{}", nome);
}
```
