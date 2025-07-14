# WebAssembly Shellcode 🦀

## Overview

This project focuses on demonstrating how we can execute shellcode using WebAssembly. It is often used for shellcode evasion.

## Usage 

First, access the "shellcode assembly" directory and, if you wish, replace your shellcode with a specific one, in my case, it is configured to run notepad.exe. Then run the command:

```sh
wasm-pack build --target bundler
```

It will generate a file called "shellcode_webassembly_bg.wasm" in the "/pkg" directory, then use the command to convert it to ".wat":

```sh
wasm2wat pkg/shellcode_webassembly_bg.wasm -o shell.wat
```

That concludes the shellcode generation part. Now, upload your shell.wat file to the directory. Next, we'll execute the file, which in the project is called "execute_shellcode", and run the command directly:
```sh
cargo run --release
```

Or by running the binary directly after generation:
```sh
target/release/execute_shellcode.exe
```

Remember that we can embed the shellcode inside the binary, but to make it more didactic we prefer to leave the "shell.wat" file separate from the binary.

## References

https://balwurk.com/shellcode-evasion-using-webassembly-and-rust/