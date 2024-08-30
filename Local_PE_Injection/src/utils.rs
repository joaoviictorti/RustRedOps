#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::{ffi::c_void, arch::asm};
use ntapi::ntpebteb::{PEB, TEB};
use windows::Win32::{
    Foundation::{BOOL, HINSTANCE}, 
    System::Kernel::NT_TIB
};

const IMAGE_ORDINAL_FLAG64: u64 = 0x8000000000000000;
pub type Main = unsafe extern "system" fn() -> BOOL;
pub type DllMain = unsafe extern "system" fn(HINSTANCE, u32, *mut c_void) -> BOOL;

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

pub fn image_snap_by_ordinal(ordinal: u64) -> bool {
    ordinal & IMAGE_ORDINAL_FLAG64 != 0
}

pub fn image_ordinal(ordinal: u64) -> u64 {
    ordinal & 0xffff
}

pub unsafe fn get_peb() -> *mut PEB {
    let teb_offset = ntapi::FIELD_OFFSET!(NT_TIB, Self_) as u32;

    #[cfg(target_arch = "x86_64")]
    {
        let teb = __readgsqword(teb_offset) as *mut TEB;
        (*teb).ProcessEnvironmentBlock
    }

    #[cfg(target_arch = "x86")]
    {
        let teb = __readfsdword(teb_offset) as *mut TEB;
        (*teb).ProcessEnvironmentBlock
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