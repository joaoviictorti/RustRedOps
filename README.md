# RustRedOps

<p align="center">
    <img height="200" alt="RustRedOps" src="rust.png">
</p>


## Summary

* [In Construction](#in-construction)
* [Overview](#overview)
* [Contents](#contents)
* [Resources](#resources)
* [Requirements](#requirements)
* [Compile](#compile)
    * [Compiling the Project](#compiling-the-project)
    * [Adding Destination Architectures](#adding-destination-architectures)
    * [Compiling for a Specific Architecture](#compiling-for-a-specific-architecture)
* [How to get started](#how-to-get-started)
* [Contributing to RustRedOps](#contributing-to-rustRedOps)
* [Credits / References / Thanks / Motivation](#credits--references--thanks--motivation)

## In Construction

The project is still under development

## Overview

RustRedOps is a repository that houses various tools and projects related to Red Team operations, developed in Rust. This repository is dedicated to providing effective and efficient tools for security professionals and penetration testers who want to perform security assessments and intrusion tests.

## Contents

The repository is organized into several projects, each with its own purpose and functionality. Here are some examples of the projects included:

1. [**APC Injection**](/APC_Injection)
   - This project exploits the Asynchronous Code Injection (APC) technique to execute malicious code in target processes.
   
2. [**Early Bird APC Injection**](/Early_Bird_APC_Injection)
   - It focuses on a variation of APC injection, executing code before the main process starts.

3. [**Local Payload Execution**](/Local_Payload_Execution)
   - This project addresses the direct execution of malicious payloads in a system's local environment.

4. [**Process Argument Spoofing**](/Process_Argument_Spoofing)
   - Exploits the technique of masking or altering the arguments of a process to hide malicious activity.
   
5. [**Process Injection (DLL)**](/Process_Injection_DLL)
   - It focuses on injecting dynamic link libraries (DLL) into running processes to execute malicious code.

6. [**Process Injection (Shellcode)**](/Process_Injection_Shellcode)
   - It exploits shellcode injection directly into running processes to control or execute malicious tasks.

7. [**Local Thread Hijacking**](/Local_Thread_Hijacking)
    - This project deals with hijacking the threads of processes running on the local system to execute malicious code.

8. [**Remote Thread Hijacking**](/Remote_Thread_Hijacking)
    - It addresses the hijacking of threads in remote system processes to carry out malicious actions.

9. [**Local Function Stomping Injection**](/Local_Function_Stomping_Injection)
    - It focuses on replacing locally running functions with malicious code, changing their default behavior.

10. [**Remote Function Stomping Injection**](/Remote_Function_Stomping_Injection)
    - It exploits the substitution of functions in remote systems to carry out malicious activities.

11. [**Local Mapping Injection**](/Local_Mapping_Injection)
    - Performing malicious code injection via memory mapping into local processes.

12. [**Remote Mapping Injection**](/Remote_Mapping_Injection/)
    - Performing malicious code injection via memory mapping into remote processes.

13. [**Minidump-rs**](/Minidump-rs)
    - Dumping the lsass.exe process.

14. [**Registry Shellcode**](/Registry_Shellcode)
    - Writing and reading shellcode to the Windows Registry.

15. [**API Hooking**](/API_Hooking)
    - Demonstration on API hooking which is a programming technique that allows you to intercept and manipulate calls to Windows API functions.

16. [**Encryption AES (Shellcode)**](/Encryption_AES_Shellcode)
    - Encrypting / Decrypting a shellcode using AES.

17. [**Encryption RC4 (Shellcode)**](/Encryption_RC4_Shellcode)
    - Encrypting / Decrypting a shellcode using RC4.

18. [**String Hashing**](/String_Hashing)
    - Creating string hashes to perform hiding.

19. [**Anti-Debug**](/Anti_Debug)
    - Techniques Anti-Debugging.

20. [**Local Payload Execution (Linux)**](/Local_Payload_Execution%20(Linux))
    - This project deals with the direct execution of malicious payloads in the local environment of a system in which we focus on linux.

21. [**Syscalls**](/Syscalls)
    - This project focuses on the injection attack in the local process, but using syscalls directly.

22. [**Anti-Analysis**](/Anti_Analysis)
    - Techniques Anti-Analysis.

23. [**Remove CRT**](/Remove_CRT)
    - It focuses on removing the CRT (C Runtime Library) from the binary.

24. [**Request Shellcode**](/Request_Shellcode)
    - Retrieving shellcode from HTTP requests using Rust.

25. [**Enumeration Process**](/Enumeration_Processes)
    - Enumerating processes with Rust.

26. [**IAT**](/IAT)
    - IAT obfuscation by replacing GetProcAddress and GetModuleHandle.

27. [**Create Driver**](/Create_Driver)
    - It's a project to demonstrate how to create a simple driver using rust.

28. [**Create DLL**](/Create_DLL)
    - It's a project to demonstrate how to create dll using rust.

29. [**WMI**](/WMI)
    - Running WMI (Windows Management Instrumentation) queries.

30. [**Enable All Tokens**](/Enable_All_Tokens)
    - Enabling all privilege tokens.

31. [**PPID Spoofing**](/PPID_Spoofing)
    - Demonstrating the PPID Spoofing technique.

32. [**Callback Code Execution**](/Callback_Code_Execution)
    - Demonstration of shellcode execution via callback.

33. [**Payload Execution Control**](/Payload_Execution_Control)
    - Controlling payload execution through Mutex, Events and Semaphores.

34. [**Parsing PE Headers**](/Parsing_PE)
    -  The code is focused on parsing the PE header of any Windows executable file.

35. [**Block DLL Policy**](/Block_DLL_Policy)
    - Avoiding the loading of DLLS not signed by Microsoft.

36. [**Execute Command**](/Execute_Command)
    - Running commands with Rust.

37. [**Payload Placement**](/Payload_Placement)
    - Storing a shellcode in the .text section and then executing it.

38. [**Payload Execution Fibers**](/Payload_Execution_Fibers)
    - Running shellcode using Fibers.

39. [**Obfuscation Shellcode**](/Obfuscation)
    - Shellcode obfuscation using IPV4, IPV6, MAC and UUIDs.

40. [**WebAssembly Shellcode**](/WebAssembly_Shellcode)
    - Running shellcode through WebAssembly.

41. [**Extract WIFI**](/Extract_Wifi)
    - Extracting WIFI passwords using winapis is a customized form of the netsh command.

42. [**Self Deletion**](/Self_Deletion)
    - Technique for deleting the running binary.

43. [**Binary Info**](/Binary_Info)
    - This is just a simple demonstration in case you want to include metadata in your Rust binary or change the associated icon.

44. [**NTDLL Unhooking**](/NTDLL_Unhooking)
    - Running NTDLL Unhooking through a suspended process.

45. [**Patch ETW**](/Patch_ETW)
    - Patching ETW.

46. [**Patch AMSI**](/Patch_AMSI)
    - Patching AMSI.

47. [**Create UEFI**](/Create_UEFI)
    - It's a project to demonstrate how to create uefi using rust.

48. [**Named Pipe Server / Client**](/Named_Pipe_Client_Server)
    - A simple project showing how we can communicate between processes using named pipes.

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
Replace <arch> with the desired architecture, such as x86_64-pc-windows-gnu.

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
To contribute to RustRedOps, follow these steps:

1. Fork this repository.
2. Create a branch: ```git checkout -b <branch_name>```.
3. Make your changes and confirm them: ```git commit -m '<commit_message>'```.
4. Send to the original branch: ```git push origin <project_name> / <local>```.
5. Create the pull request.

Alternatively, consult the GitHub documentation on how to create a pull request.

## Credits / References / Thanks / Motivation
I would like to express my sincere gratitude to the creators of remarkable projects and fascinating techniques, who provided me with the tools and inspiration needed to create this extraordinary repository.

* https://github.com/MemN0ps
* https://github.com/hasherezade
* https://github.com/vxunderground 
* https://github.com/trickster0
* https://github.com/BlWasp
* https://balwurk.com/shellcode-evasion-using-webassembly-and-rust
* https://github.com/janoglezcampos/rust_syscalls
* https://github.com/microsoft
* https://ired.team
* https://github.com/rust-osdev/uefi-rs
* https://github.com/StephanvanSchaik/windows-kernel-rs
* https://discord.gg/rust-lang-community (Discord community that helped a lot)