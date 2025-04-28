# Thread Hijacking 🦀

This repo contains two versions of Thread Hijacking:

- **Local**: hijacks a thread within the *same* process.
- **Remote**: hijacks a thread in a *remote* process (PID required).

Both techniques modify a suspended thread's context to redirect execution to custom shellcode or payload.

## Usage

Build and run using Cargo:

```sh
cargo run --release
```