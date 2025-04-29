# Obfuscation Shellcode 🦀

Shellcode obfuscation is a fundamental technique in the development of malware, with the aim of avoiding detection by antivirus software and security analysis systems.  This project uses an approach to shellcode obfuscation, incorporating IPv4 and IPv6 addressing formats, MAC addresses UUIDs and WORDS.

Each directory contains the method needed to perform de-obfuscation for IPV4, IPV6, MAC, UUID and WORDS techniques.

## Usage 

You can run with cargo run or the compiled binary directly:
```sh
cargo run --release -- -f shell.bin -t ipv4 -a obfuscate
```

If you have any doubts, you can check the help using --help:
```sh
cargo run --release -- --help
```

## Available Actions

* **obfuscate** - Encodes the original shellcode into the selected format (e.g., IPv4 addresses).
* **deobfuscate** - Decodes the obfuscated data back into the original raw shellcode.

## Examples

### Obfuscating shellcode

```sh
cargo run --release -- -f file.bin -t ipv4 -a obfuscate`
```

This will print the obfuscated shellcode to the terminal in a structured Rust vector format.

Example output:
```rs
let shellcode = vec![
    "252.72.131.228",
    "240.232.192.0",
    "0.0.65.81",
    // ...
];
```

### Deobfuscating shellcode

If you have an obfuscated file (e.g., file.txt containing the list of IP addresses), you can revert it back:

```sh
cargo run --release -- -f file.txt -t ipv4 -a deobfuscate
```

It will print the reconstructed raw shellcode as a Rust-formatted vector:

Example output:
```rs
let shellcode = vec![
    0xFC, 0x48, 0x83, 0xE4, 0xF0, 0xE8, 0xC0, 0x00,
    0x00, 0x00, 0x41, 0x51, 0x41, 0x50, 0x52, 0x51,
    // ...
];
```