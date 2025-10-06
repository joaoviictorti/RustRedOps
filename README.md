# RustRedOps

![Rust](https://img.shields.io/badge/made%20with-Rust-red)
![Platform](https://img.shields.io/badge/platform-windows-blueviolet)
![Forks](https://img.shields.io/github/forks/joaoviictorti/RustRedOps)
![Stars](https://img.shields.io/github/stars/joaoviictorti/RustRedOps)
![License](https://img.shields.io/github/license/joaoviictorti/RustRedOps)

</br>

<p align="center">
    <img height="200" width=350 alt="RustRedOps" src="rust.png">
</p>

</br>

RustRedOps is a repository that houses various tools and projects related to Red Team operations, developed in Rust. This repository is dedicated to providing effective and efficient tools for security professionals and penetration testers who want to perform security assessments and intrusion tests.

## Table of Contents

* [Contents](#contents)
  * [Other Rust Offensive Projects](#other-rust-offensive-projects)
* [Resources](#resources)
* [Requirements](#requirements)
* [Compile](#compile)
    * [Compiling the Project](#compiling-the-project)
    * [Adding Destination Architectures](#adding-destination-architectures)
    * [Compiling for a Specific Architecture](#compiling-for-a-specific-architecture)
* [How to get started](#how-to-get-started)
* [Contributing to RustRedOps](#contributing-to-rustRedOps)
* [References](#references)
    * [Other Essential Resources](#other-essential-resources)
* [License](#license)
* [Contributors](#contributors)

## Contents

The repository is organized into several projects, each with its own purpose and functionality. Here are some examples of the projects included:

1. [**APC Injection**](/APC-Injection)
   - This project exploits the Asynchronous Code Injection (APC) technique to execute malicious code in target processes.

2. [**API Hooking**](/API-Hooking)
    - Demonstration on API hooking which is a programming technique that allows you to intercept and manipulate calls to Windows API functions.
   
3. [**Anti-Debug**](/Anti-Debug)
    - Techniques Anti-Debugging.

4. [**API Hammering**](/API-Hammering)
    - API Hammering consists of carrying out various actions to delay the malware.

5. [**Anti-Analysis**](/Anti-Analysis)
    - Techniques Anti-Analysis.

6. [**Binary Info**](/Binary-Info)
    - This is just a simple demonstration in case you want to include metadata in your Rust binary or change the associated icon.

7. [**Block DLL Policy**](/Block-DLL-Policy)
    - Avoiding the loading of DLLS not signed by Microsoft.

8. [**Create Driver**](/Create-Driver)
    - It's a project to demonstrate how to create a simple driver using rust.

9. [**Create DLL**](/Create-DLL)
    - It's a project to demonstrate how to create dll using rust.

10. [**Callback Code Execution**](/Callback-Code-Execution)
    - Demonstration of shellcode execution via callback.

11. [**Create UEFI**](/Create-UEFI)
    - It's a project to demonstrate how to create uefi using rust.

12. [**Compile Encrypt String**](/Compile-Encrypt-String)
    - Encrypting strings at compile time and decrypting them at runtime.

13. [**Extract WIFI**](/Extract-Wifi)
    - Extracting WIFI passwords using winapis is a customized form of the netsh command.

14. [**Encryption (Shellcode)**](/Encryption-Shellcode)
    - Encrypting / Decrypting a shellcode using AES and RC4.

15. [**Enumeration Process**](/Enumeration-Processes)
    - Enumerating processes with Rust.

16. [**Token**](/Token)
    - Enabling all privilege tokens.

17. [**Execute Command**](/Execute-Command)
    - Running commands with Rust.

18. [**Hells / Halos / Tartarus Gate**](/Hells-Halos-Tartarus-Gate)
    - Recovering ssn through the Hells / Halos / Tartarus Gate techniques

19. [**IAT Obfuscation**](/IAT-Obfuscation)
    - IAT obfuscation by replacing GetProcAddress and GetModuleHandle.

20. [**IAT Camouflage**](/IAT-Camouflage)
    - Technique for exporting APIs (without executing them) in order to camouflage the IAT and avoid a malicious appearance.

21. [**Local Payload Execution**](/Local-Payload-Execution)
    - This project addresses the direct execution of malicious payloads in a system's local environment.

22. [**Mapping Injection**](/Mapping-Injection)
    - Code injection technique that leverages memory mapping, with the same underlying method applied to either local or remote processes

23. [**Function Stomping Injection**](/Function-Stomping-Injection)
    - Involves replacing legitimate functions with malicious code.

24. [**Thread Hijacking**](/Thread-Hijacking)
    - Focuses on hijacking existing threads to execute arbitrary code.
    
25. [**Local PE Injection**](/Local-PE-Injection)
    - Running a PE file in memory.

26. [**Minidump**](/Minidump)
    - Dumping the lsass.exe process.

27. [**Module Stomping**](/Module-Stomping)
    - The Module Stomping technique focuses on injecting a shellcode into the entrypoint of the mapped or loaded DLL.

28. [**NTDLL Unhooking**](/NTDLL-Unhooking)
    - Running NTDLL Unhooking through a suspended process.

29. [**Named Pipe Server / Client**](/Named-Pipe-Client-Server)
    - A simple project showing how we can communicate between processes using named pipes.

30. [**Module Overloading**](/Module-Overloading)
    - Module Overloading is a technique that maps a target DLL and replaces its contents with an EXE / DLL file and then executes it.

31. [**Obfuscation Shellcode**](/Obfuscation)
    - Shellcode obfuscation using IPV4, IPV6, MAC and UUIDs.

32. [**PPID Spoofing**](/PPID-Spoofing)
    - Demonstrating the PPID Spoofing technique.

33. [**Parsing PE Headers**](/Parsing-PE)
    -  The code is focused on parsing the PE header of any Windows executable file.

34. [**ETW**](/ETW)
    - Patching ETW.

35. [**AMSI**](/AMSI)
    - Patching AMSI.

36. [**Payload Execution Control**](/Payload-Execution-Control)
    - Controlling payload execution through Mutex, Events and Semaphores.

37. [**Process Argument Spoofing**](/Process-Argument-Spoofing)
    - Exploits the technique of masking or altering the arguments of a process to hide malicious activity.
   
38. [**Process Injection**](/Process-Injection)
    - Injects a DLL or shellcode into a remote process and executes.

39. [**Payload Placement**](/Payload-Placement)
    - Storing a shellcode in the .text section and then executing it.

40. [**Process Hypnosis**](/Process-Hypnosis)
    - This technique focuses on controlling the execution flow of a program that is being debugged and obtaining relevant information from it, such as the creation of new threads, loaded modules, exceptions and much more. Or even execute a shellcode.

41. [**Payload Execution Fibers**](/Payload-Execution-Fibers)
    - Running shellcode using Fibers.

42. [**Payload Staging**](/Payload-Staging)
    - This project focuses on demonstrating how to perform shellcode retrieval using an HTTP request and a Registry key

43. [**Process Ghosting**](/Process-Ghosting)
    - Loading a PE file using the Process Ghosting technique.

44. [**Process Herpaderping**](/Process-Herpaderping)
    - Obscuring the intentions of a process by modifying the contents of the disk after the image has been mapped.

45. [**CRT**](/CRT)
    - It focuses on minimizing the use of the CRT (C Runtime Library) during runtime and applying additional flags to strip away unnecessary information from the binary.

46. [**Self Deletion**](/Self-Deletion)
    - Technique for deleting the running binary.

47. [**String Hashing**](/String-Hashing)
    - Creating string hashes to perform hiding.

48. [**Syscalls**](/Syscalls)
    - Running direct and indirect syscall.

40. [**Threadless Injection**](/Threadless-Injection)
    - Performing Threadless Injection using Rust.

50. [**WMI**](/WMI)
    - Running WMI (Windows Management Instrumentation) queries.

51. [**WebAssembly Shellcode**](/WebAssembly-Shellcode)
    - Running shellcode through WebAssembly.

### Other Rust Offensive Projects

Here are some other examples of projects I've done with Rust:

- [**shadow-rs**](https://github.com/joaoviictorti/shadow-rs)
    - Windows Kernel Rootkit.
- [**coffeeldr**](https://github.com/joaoviictorti/coffeeldr)
    - COFF Loader.
- [**rustclr**](https://github.com/joaoviictorti/rustclr)
    - Host CLR and run .NET binaries.
- [**userdmp**](https://github.com/joaoviictorti/userdmp)
    - A Rust crate to parse user-mode minidump files generated on Windows.
- [**dinvk**](https://github.com/joaoviictorti/dinvk)
    - Dynamically invoke arbitrary code and use various tricks written idiomatically in Rust (Dinvoke).
- [**runas-rs**](https://github.com/joaoviictorti/runas-rs)
    - A runas implementation with extra features in Rust.
- [**uwd**](https://github.com/joaoviictorti/uwd)
    - Call Stack Spoofing for Rust.
- [**hypnus**](https://github.com/joaoviictorti/hypnus)
    - Memory Obfuscation in Rust.

## Resources

- Each individual project can include a features section that details the project's main features and functionalities.
- You can view the installation instructions, usage and examples for each project in their respective directories.

## Requirements

- [Rust](https://www.rust-lang.org/): Rust is a modern and secure programming language used to develop the tools in this repository.
- [Cargo](https://doc.rust-lang.org/cargo/): Cargo is Rust's package manager and compiler, essential for compiling and running projects.

## Compile

### Compiling the Project

To start the compilation, use the following command:

```sh
cargo build --release
```

### Adding Destination Architectures

If you are using a different operating system or need to compile for a specific architecture, you can list all available target architectures with the following command:

```sh
rustup target list
```

Once you have identified the desired target architecture, add it using rustup:

```sh
rustup target add <arch>
```

Replace `<arch>` with the desired architecture, such as `x86_64-pc-windows-gnu`.

### Compiling for a Specific Architecture

Then compile the project for the specific architecture:

```sh
cargo build --release --target <arch>
```

## How to get started

Follow these steps to start using the projects in this repository:

1. Clone this repository on your local machine:
   ```sh
   git clone https://github.com/joaoviictorti/RustRedOps.git
   ```
2. Navigate to the directory of the project you are interested in:
   ```sh
   cd RustRedOps/<name-project>
   ```
3. Follow the project-specific installation and usage instructions as described in the README inside this directory.


## Contributing to RustRedOps

To contribute to **RustRedOps**, follow these steps:

1. Fork this repository.
2. Create a branch: `git checkout -b <branch_name>`.
3. Make your changes and commit them: `git commit -m '<commit_message>'`.
4. Push your changes to your branch: `git push origin <branch_name>`.
5. Create a pull request.

Alternatively, consult the [GitHub documentation](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests) on how to create a pull request.

## References

I would like to express my sincere gratitude to the creators of remarkable projects and fascinating techniques, who provided me with the tools and inspiration needed to create this extraordinary repository.

* [MemN0ps](https://github.com/MemN0ps)
* [hasherezade](https://github.com/hasherezade)
* [vxunderground](https://github.com/vxunderground) 
* [NUL0x4C](https://github.com/NUL0x4C)
* [mrd0x](https://github.com/mrd0x)
* [Cracked5pider](https://github.com/Cracked5pider)
* [trickster0](https://github.com/trickster0)
* [BlWasp](https://github.com/BlWasp)
* [CCob](https://github.com/CCob)

### Other Essential Resources:

* https://balwurk.com/shellcode-evasion-using-webassembly-and-rust
* https://github.com/janoglezcampos/rust_syscalls
* https://github.com/microsoft
* https://ired.team
* https://github.com/rust-osdev/uefi-rs
* https://discord.gg/rust-lang-community
* https://github.com/anvie/litcrypt.rs

## License

This project is licensed under the [**MIT License**](/LICENSE)

## Contributors

[![contributors](https://contrib.rocks/image?repo=joaoviictorti/RustRedOps) ](https://github.com/joaoviictorti/RustRedOps/graphs/contributors)
