# Process Injection

Process injection is a technique that allows arbitrary code execution inside the memory space of another process. It's commonly used by both legitimate tools and malware to run code in a different execution context.

* `Shellcode Injection`: Injects raw shellcode into a remote process and executes it with `CreateRemoteThread`.
* `DLL Injection`: Injects a DLL into a remote process and forces execution via `LoadLibrary` with `CreateRemoteThread`.

## Usage 

You can run with cargo run or the compiled binary directly:

### Shellcode Injection
```sh
cargo run --release
```

### DLL Injection
```sh
cargo run --release -- <pid> <full-path-of-the-DLL>
```
