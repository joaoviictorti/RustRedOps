# Create UEFI 🦀

The repository in question is an essential starting point for security experts interested in creating UEFI (Unified Extensible Firmware Interface) using the Rust programming language.

## Usage

The folks at Rust-OSDev demonstrate how to use it on Linux. In my case, I'll be showing it on Windows.

First of all, perform the build:
```sh
cargo build --target x86_64-unknown-uefi
```
This will generate a file with the extension ".efi".

Next, use the winget command to search for qemu, which will run our UEFI inside the VM:
```sh
winget show qemu
```

When you run it, it shows some information and the interesting part is the "Installer Url", which will contain the ".exe" file needed to perform the installation. Just follow the instructions.

After installing qemu, run the command to create the "esp/boot" folder, which will contain our ".efi" file:
```sh
mkdir -p esp/efi/boot
```
Take the ".efi" file that was generated, rename it to "bootx64.efi" and move it to the "esp/boot" folder:
```sh
mv bootx64.efi esp/efi/boot
```

Finally, run "qemu":
```sh
qemu-system-x86_64 -drive if=pflash,format=raw,readonly=on,file=OVMF_CODE.fd -drive if=pflash,format=raw,readonly=on,file=OVMF_VARS.fd -drive format=raw,file=fat:rw:esp
```

Result:
![Result UEFI](img/uefi.png)

## References

* https://github.com/rust-osdev/uefi-rs
* https://rust-osdev.github.io/uefi-rs/HEAD/introduction.html