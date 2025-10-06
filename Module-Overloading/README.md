# Module Overloading 🦀

Module overloading is a method of swapping the contents of a DLL mapped to our process using the NtCreateSection and NtMapViewOfSection APIs, then replacing its contents with an EXE/DLL file and subsequently executing its entry point.

## Usage 

You can pass a DLL file or an EXE to the binary:
```sh
cargo run -- -f <file.exe / file.dll> --dll "<path-dll>"
```

You can pass arguments to your file, in this example I'm using mimikatz.exe
```sh
cargo run -- -f mimikatz.exe --dll "C:\Windows\System32\user32.dll" --args "coffee exit"
```

## References

* https://github.com/hasherezade/module_overloading