# API Hammering 🦀

API Hammering consists of a large number of useless Windows API function calls, functions, loops, writing files and so on. 

In the Rust project I present two ways of carrying out this type of action, through file writing and interactions between loops.

With this technique we can escape sandbox analyses by slowly executing our malware.

## Usage 

You can run with cargo run or the compiled binary directly:
```sh
cargo run --release
```

## References

* https://github.com/rad9800/BloatedHammer
* https://unit42.paloaltonetworks.com/api-hammering-malware-families
* https://github.com/chvancooten/maldev-for-dummies