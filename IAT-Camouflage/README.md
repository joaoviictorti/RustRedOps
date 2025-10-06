# IAT Camouflage 🦀

By working with direct/indirect syscall and removing the C runtime library, our binary will generally not have IAT exports, which can give it a malicious appearance. The technique in question aims to "trick" the compiler into including some APIs in the export process without them actually being executed at runtime. This helps mitigate the suspicious appearance of the binary, making it less likely to be marked as malicious.

## Usage

You can run with cargo run or the compiled binary directly:
```sh
cargo run --release
```