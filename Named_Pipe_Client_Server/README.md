# Named Pipe Client / Server 🦀

<p align="left">
	<a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/made%20with-Rust-red"></a>
	<a href="#"><img src="https://img.shields.io/badge/platform-windows-blueviolet"></a>
</p>

- [Overview](#overview)
- [Usage](#usage)

# Overview

This Named Pipe server and client project is a solution for communication between processes. This project aims to facilitate the exchange of data between processes in a secure and efficient way, using Named Pipes to create dedicated communication channels.

# Usage 

To run the server use:
```sh
cargo run --bin pipe_server
```

To run the client use:
```sh
cargo run --bin pipe_client
```