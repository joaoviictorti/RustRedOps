use std::{ffi::c_void, mem::size_of, ptr::null_mut};
use windows::{
    core::{Result, PSTR},
    Win32::{
        Foundation::HANDLE,
        System::{
            Memory::{GetProcessHeap, HeapAlloc, HEAP_ZERO_MEMORY},
            Threading::{
                CreateProcessA, DeleteProcThreadAttributeList, InitializeProcThreadAttributeList,
                OpenProcess, UpdateProcThreadAttribute, EXTENDED_STARTUPINFO_PRESENT,
                LPPROC_THREAD_ATTRIBUTE_LIST, PROCESS_ALL_ACCESS, PROCESS_INFORMATION,
                PROC_THREAD_ATTRIBUTE_PARENT_PROCESS, STARTUPINFOEXA,
            },
        },
    },
};

fn main() -> Result<()> {
    let mut si = STARTUPINFOEXA::default();
    let mut pi = PROCESS_INFORMATION::default();
    si.StartupInfo.cb = size_of::<STARTUPINFOEXA>() as u32;

    unsafe {
        // Open handle to the parent process by PID
        let h_parent_process = OpenProcess(PROCESS_ALL_ACCESS, false, 35372)?;
        
        // Initialize the attribute list
        let mut attr_size = 0;
        let _ = InitializeProcThreadAttributeList(
            LPPROC_THREAD_ATTRIBUTE_LIST(null_mut()),
            1,
            0,
            &mut attr_size,
        );

        let attr_list = LPPROC_THREAD_ATTRIBUTE_LIST(
            HeapAlloc(GetProcessHeap()?, HEAP_ZERO_MEMORY, attr_size)
        );

        InitializeProcThreadAttributeList(attr_list, 1, 0, &mut attr_size)?;

        // Update the attribute list with the parent process handle
        UpdateProcThreadAttribute(
            attr_list,
            0,
            PROC_THREAD_ATTRIBUTE_PARENT_PROCESS as usize,
            Some(&h_parent_process as *const _ as *const c_void),
            size_of::<HANDLE>(),
            None,
            None,
        )?;

        // Create the new process (Notepad) with spoofed parent
        let windir = std::env::var("WINDIR").unwrap_or("".to_string()) + "\\System32\\notepad.exe";
        si.lpAttributeList = attr_list;
        CreateProcessA(
            None,
            PSTR(windir.as_ptr().cast_mut()),
            None,
            None,
            false,
            EXTENDED_STARTUPINFO_PRESENT,
            None,
            None,
            &si.StartupInfo,
            &mut pi,
        )?;

        // Free allocated memory for attribute list
        DeleteProcThreadAttributeList(attr_list);
    }

    Ok(())
}
