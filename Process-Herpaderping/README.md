# Process Herpaderping 🦀

Process Herpaderping is a technique that aims to hide the true intentions of a process by altering the disk contents after the process image has already been loaded into memory. This procedure causes an unusual reaction in both security systems and the operating system itself.

## Usage 

To execute the file, pass the EXE file that will be executed:
```sh
cargo run --release -- <file.exe> <args> <path>
```

Real use:
```sh
cargo run --release -- mimikatz.exe "coffee localtime" C:\Windows\System32\OneDriveSetup.exe
```

# References

* https://github.com/jxy-s/herpaderping