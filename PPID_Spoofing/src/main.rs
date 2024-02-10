use std::{ffi::c_void, mem::size_of, ptr::null_mut};
use windows::{
    core::PSTR,
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

fn main() {
    let mut startup_info = STARTUPINFOEXA::default();
    let mut process_info = PROCESS_INFORMATION::default();
    startup_info.StartupInfo.cb = size_of::<STARTUPINFOEXA>() as u32;
    unsafe {
        let h_parent_process = OpenProcess(PROCESS_ALL_ACCESS, false, 10404).unwrap_or_else(|err| panic!("Error opening parent process: {}", err)); // PPID
        let mut attr_size: usize = 0;
        let _ = InitializeProcThreadAttributeList(
            LPPROC_THREAD_ATTRIBUTE_LIST(null_mut()),
            1,
            0,
            &mut attr_size,
        );
        let attr_list = LPPROC_THREAD_ATTRIBUTE_LIST(HeapAlloc(
            GetProcessHeap().unwrap(),
            HEAP_ZERO_MEMORY,
            attr_size,
        ));

        let _ = InitializeProcThreadAttributeList(attr_list, 1, 0, &mut attr_size);

        let _ = UpdateProcThreadAttribute(
            attr_list,
            0,
            PROC_THREAD_ATTRIBUTE_PARENT_PROCESS as usize,
            Some(&h_parent_process as *const _ as *const c_void),
            size_of::<HANDLE>(),
            None,
            None,
        );

        let windir = std::env::var("WINDIR").unwrap() + "\\System32\\notepad.exe";
        startup_info.lpAttributeList = attr_list;
        let _ = CreateProcessA(
            None,
            PSTR(windir.as_ptr() as _),
            None,
            None,
            false,
            EXTENDED_STARTUPINFO_PRESENT,
            None,
            None,
            &startup_info.StartupInfo,
            &mut process_info,
        );

        DeleteProcThreadAttributeList(attr_list);
    }
}
