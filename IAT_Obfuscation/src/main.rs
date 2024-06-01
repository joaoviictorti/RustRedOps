use ntapi::{
    ntldr::LDR_DATA_TABLE_ENTRY,
    ntpebteb::{PEB, TEB},
    winapi::ctypes::c_void,
};
use std::{arch::asm, ffi::CStr, slice};
use windows::Win32::System::{
    Diagnostics::Debug::IMAGE_NT_HEADERS64,
    Kernel::NT_TIB,
    SystemServices::{
        IMAGE_DOS_HEADER, IMAGE_DOS_SIGNATURE, IMAGE_EXPORT_DIRECTORY, IMAGE_NT_SIGNATURE,
    },
};

fn main() {
    unsafe {
        let address = get_module("ntdll.dll".to_string()).expect("Error obtaining module address");
        get_proc(address);
    };
}

unsafe fn get_proc(dll_base: *mut c_void) {
    let dos_header = dll_base as *mut IMAGE_DOS_HEADER;
    if (*dos_header).e_magic != IMAGE_DOS_SIGNATURE {
        panic!("INVALID DOS SIGNATURE");
    }

    let nt_header = (dll_base as usize + (*dos_header).e_lfanew as usize) as *mut IMAGE_NT_HEADERS64;
    if (*nt_header).Signature != IMAGE_NT_SIGNATURE {
        panic!("INVALID NT SIGNATURE");
    }

    let export_directory = (dll_base as usize + (*nt_header).OptionalHeader.DataDirectory[0].VirtualAddress as usize) as *const IMAGE_EXPORT_DIRECTORY;
    let names = (dll_base as usize + (*export_directory).AddressOfNames as usize) as *const u32;
    let ordinals = (dll_base as usize + (*export_directory).AddressOfNameOrdinals as usize) as *const u16;
    let addresss = (dll_base as usize + (*export_directory).AddressOfFunctions as usize) as *const u32;

    for i in 0..(*export_directory).NumberOfNames as isize {
        let name = CStr::from_ptr((dll_base as usize + *names.offset(i) as usize) as *const i8).to_str().unwrap();
        let ordinal = *ordinals.offset(i);
        let address = (dll_base as usize + *addresss.offset(ordinal as isize) as usize) as *mut c_void;
        println!("NAME {} | ADDRESS: {:?} | ORDINAL: {}", name, address, ordinal);
    }
}

unsafe fn get_module(dll: String) -> Result<*mut c_void, ()> {
    let peb = get_peb();
    let ldr = (*peb).Ldr;
    let mut list_entry = (*ldr).InLoadOrderModuleList.Flink as *mut LDR_DATA_TABLE_ENTRY;

    while !(*list_entry).DllBase.is_null() {
        let buffer = slice::from_raw_parts(
            (*list_entry).BaseDllName.Buffer,
            ((*list_entry).BaseDllName.Length / 2) as usize,
        );
        let dll_name = String::from_utf16(&buffer)
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
        let teb = __readgsqword(teb_offset) as *mut TEB;
        return (*teb).ProcessEnvironmentBlock;
    }

    #[cfg(target_arch = "x86")]
    {
        let teb = __readfsdword(teb_offset) as *mut TEB;
        return (*teb).ProcessEnvironmentBlock;
    }
}

#[cfg(target_arch = "x86_64")]
unsafe fn __readgsqword(offset: u32) -> u64 {
    let output: u64;
    asm!(
        "mov {}, gs:[{:e}]",
        lateout(reg) output,
        in(reg) offset,
        options(nostack, pure, readonly),
    );
    output
}

#[cfg(target_arch = "x86")]
unsafe fn __readfsdword(offset: u32) -> u32 {
    let output: u32;
    asm!(
        "mov {:e}, fs:[{:e}]",
        lateout(reg) output,
        in(reg) offset,
        options(nostack, pure, readonly),
    );
    output
}
