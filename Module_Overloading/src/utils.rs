#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::ffi::c_void;
use ntapi::ntpebteb::{PEB, TEB};
use windows::Win32::{
    Foundation::{BOOL, HINSTANCE}, 
    System::{
        Kernel::NT_TIB,
        Diagnostics::Debug::*, 
    }
};

const IMAGE_ORDINAL_FLAG64: u64 = 0x8000000000000000;
pub type Exe = unsafe extern "system" fn() -> BOOL;
pub type Dll = unsafe extern "system" fn(HINSTANCE, u32, *mut c_void) -> BOOL;

/// Struct representing the Portable Executable (PE) format.
#[derive(Debug)]
pub struct PE {
    /// The buffer containing the PE file data.
    pub file_buffer: Vec<u8>,
    /// Pointer to the NT headers of the PE file.
    pub nt_header: *mut IMAGE_NT_HEADERS64,
    /// Pointer to the section headers of the PE file.
    pub section_header: *mut IMAGE_SECTION_HEADER,
    /// Data directory entry for the import table.
    pub entry_import_data: IMAGE_DATA_DIRECTORY,
    /// Data directory entry for the base relocation table.
    pub entry_basereloc_data: IMAGE_DATA_DIRECTORY,
    /// Boolean indicating if the PE is a DLL.
    pub is_dll: bool,
}

/// Struct representing a base relocation entry.
#[derive(Debug, Clone, Copy)]
pub struct BASE_RELOCATION_ENTRY {
    pub data: u16,
}

impl BASE_RELOCATION_ENTRY {
    pub fn offset(&self) -> u16 {
        self.data & 0x0FFF
    }

    pub fn type_(&self) -> u16 {
        (self.data >> 12) & 0xF
    }
}

/// Determines if an image is snapped by ordinal.
pub fn image_snap_by_ordinal(ordinal: u64) -> bool {
    ordinal & IMAGE_ORDINAL_FLAG64 != 0
}

/// Extracts the ordinal value from an image ordinal.
pub fn image_ordinal(ordinal: u64) -> u64 {
    ordinal & 0xffff
}

/// Retrieves the Process Environment Block (PEB) of the current process.
pub unsafe fn get_peb() -> *mut PEB {
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
