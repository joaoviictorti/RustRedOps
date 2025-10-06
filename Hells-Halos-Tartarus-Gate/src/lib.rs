pub mod syscall;

#[cfg(test)]
mod test {
    use crate::syscall::get_ssn;

    const ALLOCATE_VIRTUAL_MEMORY_HASH: u32 = 1737737036;
    const WRITE_VIRTUAL_MEMORY_HASH: u32 = 2515773330;
    const PROTECT_VIRTUAL_MEMORY_HASH: u32 = 136929992;
    const CREATE_THREAD_EX_HASH_HASH: u32 = 3406569776;

    #[test]
    fn find_ssn() {
        env_logger::init();

        log::debug!("NtAllocateVirtualMemory | SSN: {}", get_ssn(ALLOCATE_VIRTUAL_MEMORY_HASH));
        log::debug!("NtWriteVirtualMemory | SSN: {}", get_ssn(WRITE_VIRTUAL_MEMORY_HASH));
        log::debug!("NtProtectVirtualMemory | SSN: {}", get_ssn(PROTECT_VIRTUAL_MEMORY_HASH));
        log::debug!("NtCreateThreadEx | SSN: {}", get_ssn(CREATE_THREAD_EX_HASH_HASH));
    }
}