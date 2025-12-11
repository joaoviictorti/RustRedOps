# Thread Hijacking

> Thread Hijacking is a code injection technique that suspends an existing thread, modifies its execution context (e.g., the instruction pointer), and redirects it to execute custom shellcode.

- **Local**: hijacks a thread within the *same* process.
- **Remote**: hijacks a thread in a *remote* process (PID required).
