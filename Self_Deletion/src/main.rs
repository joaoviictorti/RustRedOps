use std::{
    ffi::c_void,
    mem::{size_of, size_of_val},
};
use windows::core::PCWSTR;
use windows::Win32::{
    Storage::FileSystem::{
        FileDispositionInfo, FileRenameInfo, DELETE, FILE_DISPOSITION_INFO, FILE_FLAGS_AND_ATTRIBUTES,
        FILE_SHARE_READ, OPEN_EXISTING, SYNCHRONIZE,
    },
    Foundation::CloseHandle,
    Storage::FileSystem::{CreateFileW, SetFileInformationByHandle, FILE_RENAME_INFO},
    System::Memory::{GetProcessHeap, HeapAlloc, HeapFree, HEAP_ZERO_MEMORY},
};

fn main() {
    let stream = ":victor";
    let stream_wide: Vec<u16> = stream.encode_utf16().chain(std::iter::once(0)).collect();

    unsafe {
        let mut delete_file = FILE_DISPOSITION_INFO::default();
        let lenght = size_of::<FILE_RENAME_INFO>() + (stream_wide.len() * size_of::<u16>());
        let rename_info = HeapAlloc(GetProcessHeap().unwrap(), HEAP_ZERO_MEMORY, lenght) as *mut FILE_RENAME_INFO;

        delete_file.DeleteFile = true.into();
        (*rename_info).FileNameLength = (stream_wide.len() * size_of::<u16>()) as u32 - 2;

        std::ptr::copy_nonoverlapping(
            stream_wide.as_ptr(),
            (*rename_info).FileName.as_mut_ptr(),
            stream_wide.len(),
        );

        let path = std::env::current_exe().unwrap();
        let path_str = path.to_str().unwrap();
        let mut full_path: Vec<u16> = path_str.encode_utf16().collect();
        full_path.push(0);
        
        let mut h_file = CreateFileW(
            PCWSTR(full_path.as_ptr()),
            DELETE.0 | SYNCHRONIZE.0,
            FILE_SHARE_READ,
            None,
            OPEN_EXISTING,
            FILE_FLAGS_AND_ATTRIBUTES(0),
            None,
        ).unwrap_or_else(|e| panic!("[!] CreateFileW Failed With Error: {e}"));

        SetFileInformationByHandle(
            h_file,
            FileRenameInfo,
            rename_info as *const c_void,
            lenght as u32,
        ).unwrap_or_else(|e| panic!("SetFileInformationByHandle Failed With Error: {e}"));

        CloseHandle(h_file);

        h_file = CreateFileW(
            PCWSTR(full_path.as_ptr()),
            DELETE.0 | SYNCHRONIZE.0,
            FILE_SHARE_READ,
            None,
            OPEN_EXISTING,
            FILE_FLAGS_AND_ATTRIBUTES(0),
            None,
        ).unwrap_or_else(|e| panic!("[!] CreateFileW (2) Failed With Error: {e}"));

        SetFileInformationByHandle(
            h_file,
            FileDispositionInfo,
            &delete_file as *const FILE_DISPOSITION_INFO as _,
            size_of_val(&delete_file) as u32,
        ).unwrap_or_else(|e| panic!("SetFileInformationByHandle (2) Failed With Error: {e}"));

        CloseHandle(h_file);

        HeapFree(
            GetProcessHeap().unwrap(),
            HEAP_ZERO_MEMORY,
            Some(rename_info as *const c_void),
        );
    }
}
