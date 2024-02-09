# Create Driver 🦀

<p align="left">
	<a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/made%20with-Rust-red"></a>
	<a href="#"><img src="https://img.shields.io/badge/platform-windows-blueviolet"></a>
</p>

- [Overview](#overview)
- [Usage](#usage)
- [References](#references)

# Overview
The repository in question serves as an essential starting point for security specialists interested in creating drivers using the Rust programming language.

# Usage
To create the driver, we need to fulfill some preliminary requirements:

* Check the `Makefile.toml` file, and you'll notice a defined environment variable called `VC_BUILD_DIR`, which contains the `vcvars64.bat` script. This script is part of the Visual C++ toolkit included in Visual Studio. When run, it configures the command line environment to use Visual Studio's compilation tools by setting various environment variables. This includes the paths to compilers, libraries and other essential tools for compiling and linking programs.

* Install cargo-make to be able to build the driver, performing the tasks defined.

	`cargo install cargo-make`

* After these steps, we can use the following command to start building the program.
	
	`cargo make sign`

# References
* https://github.com/StephanvanSchaik/windows-kernel-rs