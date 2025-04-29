# Payload Placement 🦀

Storing a shellcode in the .text section and then executing it.
The .text can be executed directly because it stores variables with executable memory permissions.

## Usage 
You can run with cargo run or the compiled binary directly:
```sh
cargo run --release
```
