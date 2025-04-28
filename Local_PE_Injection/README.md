# Local PE Injection 🦀

Local PE Injection is a method to execute a PE file in memory.

## Usage 

You can pass a DLL file or an EXE to the binary:
```sh
cargo run -- --pe <file.exe / file.dll> --export <function>"
```

You can pass arguments to your file, in this example I'm using mimikatz.exe
```sh
cargo run -- --pe mimikatz.exe --arg "coffee exit"
```
Using DLL:
```sh
cargo run --release -- --pe .\main.dll --export "test"
```
