# Function Stomping Injection

> Function Stomping is a code injection technique that overwrites the beginning of a target function with custom shellcode, effectively "stomping" its original code.

- **Local**: overwrites a function in the current process.
- **Remote**: overwrites a function in a remote process (targeting a specific PID).

Both techniques replace the memory of an existing function (e.g., `MessageBoxA`) with custom shellcode and execute it.
