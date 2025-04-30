# Process Ghosting 🦀

Process Ghosting is a technique that uses a temporary file with pending deletion to create a process from that content.

## Usage 

To execute the file, pass the EXE file that will be executed:
```sh
cargo run --release -- <file.exe>
```
```sh
cargo run --release -- <file.exe> <args>
```

Real use:
```sh
cargo run --release -- mimikatz.exe "coffee localtime"
```

## References

* https://github.com/hasherezade/process_ghosting