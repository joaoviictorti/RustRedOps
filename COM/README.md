# COM 🦀

This folder is related to techniques using COM Interfaces

* **IActiveScript**: Executes VBScript or JScript code inside the current process through the Windows Script engine.
* **IBackgroundCopyManager**: Transfers files through the Background Intelligent Transfer Service (BITS).
* **IHxHelpPaneServer**: Creates a Help Pane process that can be abused to spawn executables via `file://` paths.
* **IHxInteractiveUser**: Similar to `IHxHelpPaneServer`, can be used to trigger a process creation via Help interaction.
* **IShellDispatch**: Executes system commands or programs through the Windows Shell (`ShellExecute`).
* **IShellLink**: Creates or manipulates Windows shortcut (`.lnk`) files to point to arbitrary executables.
* **IWinHttpRequest**: Performs HTTP/HTTPS requests directly from COM, allowing data exfiltration or remote command retrieval.
* **IXMLDOMDocument**: Executes XSLT scripts or transforms XML data that can contain embedded code for command execution.
