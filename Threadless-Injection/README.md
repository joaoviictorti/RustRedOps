# Threadless Injection

The Threadless Injection technique is very similar to Function Stomping Injection, the difference being that Threadless searches for a memory hole to install the shellcode, removes the trampolin installed in the target function and returns the original bytes. 

## Usage 

You can run with cargo run or the compiled binary directly (Don't forget to change the DLL and function in the code to what you want):
```sh
cargo run --release -- <process_name>
```

If you want a direct execution to observe the functionality more quickly, inject it into the powershell.exe process, because when you call it, the `AmsiScanBuffer` will be triggered and you will be able to see the injection being executed.
```sh
cargo run --release -- powershell.exe
```

# References

* https://github.com/CCob/ThreadlessInject