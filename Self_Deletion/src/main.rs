use std::ffi::c_void;
use windows::core::PCWSTR;
use windows::Win32::{
    Foundation::CloseHandle,
    Storage::FileSystem::{
        FILE_SHARE_READ, OPEN_EXISTING, SYNCHRONIZE,
        FileDispositionInfo, FileRenameInfo, DELETE, 
        FILE_DISPOSITION_INFO, FILE_FLAGS_AND_ATTRIBUTES,
    },
    System::Memory::{
        GetProcessHeap, HeapAlloc, 
        HeapFree, HEAP_ZERO_MEMORY
    },
    Storage::FileSystem::{
        CreateFileW, FILE_RENAME_INFO, 
        SetFileInformationByHandle, 
    },
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stream = ":victor";
    let stream_wide = stream.encode_utf16().chain(Some(0)).collect::<Vec<u16>>();

    unsafe {
        let mut delete_file = FILE_DISPOSITION_INFO::default();
        let lenght = size_of::<FILE_RENAME_INFO>() + (stream_wide.len() * size_of::<u16>());
        let rename_info = HeapAlloc(GetProcessHeap()?, HEAP_ZERO_MEMORY, lenght) as *mut FILE_RENAME_INFO;

        delete_file.DeleteFile = true.into();
        (*rename_info).FileNameLength = (stream_wide.len() * size_of::<u16>()) as u32 - 2;

        std::ptr::copy_nonoverlapping(
            stream_wide.as_ptr(),
            (*rename_info).FileName.as_mut_ptr(),
            stream_wide.len(),
        );

        let path = std::env::current_exe()?;
        let path_str = path.to_str().ok_or_else(|| "Error when converting to str")?;
        let full_path  = path_str.encode_utf16().chain(Some(0)).collect::<Vec<u16>>();
        
        let mut h_file = CreateFileW(
            PCWSTR(full_path.as_ptr()),
            DELETE.0 | SYNCHRONIZE.0,
            FILE_SHARE_READ,
            None,
            OPEN_EXISTING,
            FILE_FLAGS_AND_ATTRIBUTES(0),
            None,
        )?;

        SetFileInformationByHandle(
            h_file,
            FileRenameInfo,
            rename_info as *const c_void,
            lenght as u32,
        )?;

        CloseHandle(h_file)?;

        h_file = CreateFileW(
            PCWSTR(full_path.as_ptr()),
            DELETE.0 | SYNCHRONIZE.0,
            FILE_SHARE_READ,
            None,
            OPEN_EXISTING,
            FILE_FLAGS_AND_ATTRIBUTES(0),
            None,
        )?;

        SetFileInformationByHandle(
            h_file,
            FileDispositionInfo,
            &delete_file as *const FILE_DISPOSITION_INFO as _,
            std::mem::size_of_val(&delete_file) as u32,
        )?;

        CloseHandle(h_file)?;

        HeapFree(
            GetProcessHeap()?,
            HEAP_ZERO_MEMORY,
            Some(rename_info as *const c_void),
        )?;
    }

    Ok(())
}
