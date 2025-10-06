# Function Stomping Injection 🦀

This repository contains two versions of Function Stomping:

- **Local**: overwrites a function in the current process.
- **Remote**: overwrites a function in a remote process (targeting a specific PID).

Both techniques replace the memory of an existing function (e.g., `MessageBoxA`) with custom shellcode and execute it.

## Usage 

You can run with cargo run or the compiled binary directly:
```sh
cargo run --release
```
