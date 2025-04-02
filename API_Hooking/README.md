# API Hooking 🦀

API hooking in Windows, calls to these APIs are intercepted, allowing them to be monitored, modified or redirected. This is done by inserting intermediate code (the "hook") between the function call and its actual execution. When an application calls an API function, the hook is triggered first, allowing the operation to be manipulated.

## Running

Performing the API Hooking technique in MessageBoxA:
![Hook Enabled](img//hook_enabled.png)

Checking the debugger for the change:
![Debugger Enabled](img/debugger_enabled.png)

Eliminating the API hook:
![Hook Disabled](img/hook_disabled.png)

Checking the debugger when disabling the hook:
![Debugger Disabled](img/debugger_disabled.png)

## Usage 

You can run with cargo run or the compiled binary directly:
```sh
cargo run --release
```
