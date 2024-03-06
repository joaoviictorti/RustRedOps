use windows::core::s;
use windows::Win32::Foundation::CloseHandle;
use windows::Win32::Storage::FileSystem::{ReadFile, PIPE_ACCESS_DUPLEX};
use windows::Win32::System::Pipes::{
    ConnectNamedPipe, CreateNamedPipeA, PIPE_READMODE_MESSAGE, PIPE_TYPE_MESSAGE,
    PIPE_UNLIMITED_INSTANCES, PIPE_WAIT,
};

fn main() {
    unsafe {
        let h_pipe = CreateNamedPipeA(
            s!("\\\\.\\pipe\\Teste"),
            PIPE_ACCESS_DUPLEX,
            PIPE_TYPE_MESSAGE | PIPE_READMODE_MESSAGE | PIPE_WAIT,
            PIPE_UNLIMITED_INSTANCES,
            2044,
            2044,
            0,
            None,
        )
        .unwrap_or_else(|e| {
            panic!("[!] CreateNamedPipeA Failed With Error: {e}");
        });

        let mut number_return = 0;

        println!("[+] Waiting For Data");
        ConnectNamedPipe(h_pipe, None).unwrap_or_else(|e| panic!("[!] ConnectNamedPipe Failed With Error: {e}"));

        let mut buffer = [0u8; 276];

        ReadFile(h_pipe, Some(&mut buffer), Some(&mut number_return), None).unwrap_or_else(|e| panic!("[!] ConnectNamedPipe Failed With Error: {e}"));

        println!("{:?}", buffer);

        CloseHandle(h_pipe);
    }
}
