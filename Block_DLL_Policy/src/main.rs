use std::{ptr::null_mut, ffi::c_void};
use windows::core::PSTR;
use windows::Win32::System::{
    Memory::{GetProcessHeap, HeapAlloc, HEAP_ZERO_MEMORY},
    SystemServices::{
        PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY, PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY_0,
    },
    Threading::{
        CreateProcessA, DeleteProcThreadAttributeList, InitializeProcThreadAttributeList,
        ProcessSignaturePolicy, SetProcessMitigationPolicy, UpdateProcThreadAttribute,
        EXTENDED_STARTUPINFO_PRESENT, LPPROC_THREAD_ATTRIBUTE_LIST, PROCESS_INFORMATION,
        PROCESS_MITIGATION_POLICY, PROC_THREAD_ATTRIBUTE_MITIGATION_POLICY, STARTUPINFOEXA,
        STARTUPINFOW_FLAGS,
    },
};

const PROCESS_CREATION_MITIGATION_POLICY_BLOCK_NON_MICROSOFT_BINARIES_ALWAYS_ON: u64 = 0x00000001u64 << 44;

fn main() {
    create_process_block_dll();
    // current_process_block_dll();
}

fn current_process_block_dll() {
    unsafe {
        let mut policy = PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY {
            Anonymous: PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY_0 { Flags: 0 },
        };
        policy.Anonymous.Flags |= 1 << 0;
        let _ = SetProcessMitigationPolicy(
            PROCESS_MITIGATION_POLICY(ProcessSignaturePolicy.0),
            &policy as *const _ as *const _,
            std::mem::size_of_val(&policy),
        );
    }
}

fn create_process_block_dll() {
    let mut process_information = PROCESS_INFORMATION::default();
    let mut startup_info = STARTUPINFOEXA::default();
    startup_info.StartupInfo.cb = std::mem::size_of::<STARTUPINFOEXA>() as u32;
    startup_info.StartupInfo.dwFlags = STARTUPINFOW_FLAGS(EXTENDED_STARTUPINFO_PRESENT.0);
    let mut attr_size: usize = 0;
    unsafe {
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

        let policy = PROCESS_CREATION_MITIGATION_POLICY_BLOCK_NON_MICROSOFT_BINARIES_ALWAYS_ON;
        let _ = UpdateProcThreadAttribute(
            attr_list,
            0,
            PROC_THREAD_ATTRIBUTE_MITIGATION_POLICY as usize,
            Some(&policy as *const _ as *const c_void),
            std::mem::size_of::<u64>(),
            None,
            None,
        );

        let windir = std::env::var("WINDIR").unwrap() + "\\System32\\SystemSettingsBroker.exe";
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
            &mut process_information,
        );

        DeleteProcThreadAttributeList(attr_list);
    }
}
