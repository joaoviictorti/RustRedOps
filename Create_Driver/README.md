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

* Install cargo-make to be able to build the driver, performing the tasks defined.
    
	`cargo install cargo-make`

* After these steps, we can use the following command to start building the program (For the first build it is necessary to build as administrator, but for the rest of the builds it will no longer be necessary).
    
	`cargo make`


# References
* https://github.com/microsoft/windows-drivers-rs