# Mapping Injection

> Mapping Injection is a code injection technique that uses memory-mapped files (file mappings) to allocate executable memory and inject shellcode.

- **Local**: Injects shellcode into the current process by creating a memory-mapped file locally and executing it via a new thread.
- **Remote**: Injects shellcode into a remote process by creating a shared memory-mapped file, mapping it into the remote address space, and executing it remotely.
