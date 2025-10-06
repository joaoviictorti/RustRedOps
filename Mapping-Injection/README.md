# Mapping Injection🦀

This repo contains two versions of Mapping Injection:

- **Local**: Injects shellcode into the current process by creating a memory-mapped file locally and executing it via a new thread.
- **Remote**: Injects shellcode into a remote process by creating a shared memory-mapped file, mapping it into the remote address space, and executing it remotely.

## Usage

Build and run using Cargo:

```sh
cargo run --release
```