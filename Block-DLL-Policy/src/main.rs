use std::{ptr::null_mut, ffi::c_void};
use windows::core::{PSTR, Result};
use windows::Win32::System::{
    Threading::*,
    Memory::{GetProcessHeap, HeapAlloc, HEAP_ZERO_MEMORY},
    SystemServices::{
        PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY, 
        PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY_0,
    },
};

/// Constant for enabling DLL Signature Blocking for non-Microsoft binaries
const PROCESS_CREATION_MITIGATION_POLICY_BLOCK_NON_MICROSOFT_BINARIES_ALWAYS_ON: u64 = 0x00000001u64 << 44;

/// Method selector: `1` = create new process, `else` = apply to current process
const METHOD: u32 = 1;

fn main() -> Result<()> {
    if METHOD == 1 {
        create_process_block_dll()?;
    } else {
        current_process_block_dll()?;
    }

    Ok(())
}

/// Applies binary signature mitigation to the current process.
///
/// This enables blocking of all DLLs that are not signed by Microsoft in the current process context.
/// 
/// # Errors
///
/// Returns a [`windows::core::Error`] if the mitigation policy fails to apply. 
fn current_process_block_dll() -> Result<()> {
    unsafe {
        let mut policy = PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY {
            Anonymous: PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY_0 { Flags: 0 },
        };

        // Set "Microsoft Signed Only" flag
        policy.Anonymous.Flags |= 1 << 0;
        SetProcessMitigationPolicy(
            PROCESS_MITIGATION_POLICY(ProcessSignaturePolicy.0),
            &policy as *const _ as *const _,
            size_of_val(&policy),
        )?;
    }

    Ok(())
}

/// Spawns a new process with DLL signature enforcement enabled.
///
/// Applies the mitigation policy that blocks non-Microsoft signed DLLs to a child process.
/// This uses the `PROC_THREAD_ATTRIBUTE_MITIGATION_POLICY` extended startup info flag.
/// 
/// # Errors
///
/// Returns a [`windows::core::Error`] if any of the process attribute, heap allocation,
/// or process creation steps fail.
fn create_process_block_dll() -> Result<()> {
    let mut size = 0;
    let mut pi = PROCESS_INFORMATION::default();
    let mut si = STARTUPINFOEXA::default();
    si.StartupInfo.cb = size_of::<STARTUPINFOEXA>() as u32;
    si.StartupInfo.dwFlags = STARTUPINFOW_FLAGS(EXTENDED_STARTUPINFO_PRESENT.0);

    unsafe {
        // First call gets required buffer size
        InitializeProcThreadAttributeList(
            LPPROC_THREAD_ATTRIBUTE_LIST(null_mut()),
            1,
            0,
            &mut size,
        )?;

        // Allocate attribute list
        let attr_list = LPPROC_THREAD_ATTRIBUTE_LIST(HeapAlloc(
            GetProcessHeap()?,
            HEAP_ZERO_MEMORY,
            size,
        ));

        InitializeProcThreadAttributeList(attr_list, 1, 0, &mut size)?;

        // Apply the mitigation policy to block non-Microsoft binaries
        let policy = PROCESS_CREATION_MITIGATION_POLICY_BLOCK_NON_MICROSOFT_BINARIES_ALWAYS_ON;
        UpdateProcThreadAttribute(
            attr_list,
            0,
            PROC_THREAD_ATTRIBUTE_MITIGATION_POLICY as usize,
            Some(&policy as *const _ as *const c_void),
            size_of::<u64>(),
            None,
            None,
        )?;

        // Prepare process launch path (SystemSettingsBroker.exe)
        let windir = std::env::var("WINDIR").unwrap() + "\\System32\\SystemSettingsBroker.exe";
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

        DeleteProcThreadAttributeList(attr_list);
    }

    Ok(())
}
