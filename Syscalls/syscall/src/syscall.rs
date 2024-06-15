// References: https://github.com/janoglezcampos/rust_syscalls/blob/main/src/syscall.rs

#[allow(unused_imports)]
use core::arch::global_asm;

/// Macro to invoke a system call directly by its function name.
///
/// # Parameters
///
/// - `$function_name`: The name of the function as a string.
/// - `$y`: The arguments to be passed to the system call.
///
/// # Example
///
/// ```rust
/// syscall!("NtQueryInformationProcess", process_handle, process_info_class, process_info, process_info_length, return_length);
/// ```
#[cfg(all(feature = "_DIRECT_", not(feature = "_INDIRECT_")))]
#[macro_export]
macro_rules! syscall {
    ($function_name:expr, $($y:expr), +) => {
        {
        let (ssn, _) = $crate::syscall_resolve::get_ssn($crate::hash!($function_name));
        let mut cnt:u32 = 0;
        $(
            let _ = $y;
            cnt += 1;
        )+
        $crate::syscall::do_syscall(ssn, cnt, $($y), +)
    }}
}

/// Macro to invoke a system call indirectly by its function name and address.
///
/// # Parameters
///
/// - `$function_name`: The name of the function as a string.
/// - `$y`: The arguments to be passed to the system call.
///
/// # Example
///
/// ```rust
/// syscall!("NtQueryInformationProcess", process_handle, process_info_class, process_info, process_info_length, return_length);
/// ```
#[cfg(all(feature = "_INDIRECT_", not(feature = "_DIRECT_")))]
#[macro_export]
macro_rules! syscall {
    ($function_name:expr, $($y:expr), +) => {
        {
        let (ssn, addr) = $crate::syscall_resolve::get_ssn($crate::hash!($function_name));
        let mut cnt:u32 = 0;
        $(
            let _ = $y;
            cnt += 1;
        )+
        $crate::syscall::do_syscall(ssn, addr, cnt, $($y), +)
    }}
}

#[cfg(target_arch = "x86_64")]
#[cfg(all(feature = "_DIRECT_", not(feature = "_INDIRECT_")))]
global_asm!("
.global do_syscall

.section .text

do_syscall:

    mov [rsp - 0x8],  rsi
    mov [rsp - 0x10], rdi

    mov eax, ecx
    mov rcx, rdx

    mov r10, r8
    mov rdx, r9
    
    mov  r8,  [rsp + 0x28]
    mov  r9,  [rsp + 0x30]

    sub rcx, 0x4
    jle skip

    lea rsi,  [rsp + 0x38]
    lea rdi,  [rsp + 0x28]

    rep movsq
skip:
    syscall

    mov rsi, [rsp - 0x8]
    mov rdi, [rsp - 0x10]

    ret
");

#[cfg(target_arch = "x86_64")]
#[cfg(all(feature = "_INDIRECT_", not(feature = "_DIRECT_")))]
global_asm!("
.global do_syscall

.section .text

do_syscall:
    mov [rsp - 0x8],  rsi
    mov [rsp - 0x10], rdi
    mov [rsp - 0x18], r12

    mov eax, ecx
    mov r12, rdx
    mov rcx, r8

    mov r10, r9
    mov  rdx,  [rsp + 0x28]
    mov  r8,   [rsp + 0x30]
    mov  r9,   [rsp + 0x38]

    sub rcx, 0x4
    jle skip

    lea rsi,  [rsp + 0x40]
    lea rdi,  [rsp + 0x28]

    rep movsq
skip:

    mov rcx, r12

    mov rsi, [rsp - 0x8]
    mov rdi, [rsp - 0x10]
    mov r12, [rsp - 0x18]

    jmp rcx
");

#[cfg(target_arch = "x86_64")]
#[cfg(all(feature = "_DIRECT_", not(feature = "_INDIRECT_")))]
extern "C" {
    pub fn do_syscall(
        ssn: u16,
        n_args: u32,
        ...
    ) -> i32;
}

#[cfg(target_arch = "x86_64")]
#[cfg(all(feature = "_INDIRECT_", not(feature = "_DIRECT_")))]
extern "C" {
    pub fn do_syscall(
        ssn: u16,
        syscall_addr: u64,
        n_args: u32,
        ...
    ) -> i32;
}
