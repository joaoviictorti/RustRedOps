#![allow(dead_code)]

mod hash;
mod syscall_resolve;

const ALLOCATE_VIRTUAL_MEMORY_HASH: u32 = 1737737036;
const WRITE_VIRTUAL_MEMORY_HASH: u32 = 2515773330;
const PROTECT_VIRTUAL_MEMORY_HASH: u32 = 136929992;
const CREATE_THREAD_EX_HASH_HASH: u32 = 3406569776;

#[cfg(test)]
mod test {
    use crate::syscall_resolve::get_ssn;
    use super::*;

    #[test]
    fn find_ssn() {
        env_logger::init();

        let nt_allocate_virtual_memory = unsafe { get_ssn(ALLOCATE_VIRTUAL_MEMORY_HASH) };
        let nt_write_virtual_memory = unsafe { get_ssn(WRITE_VIRTUAL_MEMORY_HASH) };
        let nt_protect_virtual_memory = unsafe { get_ssn(PROTECT_VIRTUAL_MEMORY_HASH) };
        let nt_create_thread_ex =unsafe { get_ssn(CREATE_THREAD_EX_HASH_HASH) };

        log::debug!(
            r#"- NtAllocateVirtualMemory | SSN: {}
        "#,
            nt_allocate_virtual_memory
        );

        log::debug!(
            r#"- NtWriteVirtualMemory | SSN: {}
        "#,
            nt_write_virtual_memory
        );

        log::debug!(
            r#"- NtProtectVirtualMemory | SSN: {}
        "#,
            nt_protect_virtual_memory
        );

        log::debug!(
            r#"- NtCreateThreadEx | SSN: {}
        "#,
            nt_create_thread_ex
        );
    }
}