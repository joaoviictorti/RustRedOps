use windows::Win32::System::Diagnostics::Debug::*;
use windows::Win32::System::{
    Diagnostics::Debug::IMAGE_NT_OPTIONAL_HDR_MAGIC,
    SystemInformation::IMAGE_FILE_MACHINE_I386,
    SystemServices::{IMAGE_DOS_HEADER, IMAGE_DOS_SIGNATURE, IMAGE_NT_SIGNATURE},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().collect::<Vec<String>>();
    let buffer = std::fs::read(&args[1])?;
    
    unsafe {
        let dos_header = buffer.as_ptr() as *mut IMAGE_DOS_HEADER;
        if (*dos_header).e_magic != IMAGE_DOS_SIGNATURE {
            panic!("[!] Invalid IMAGE_DOS_SIGNATURE");
        }

        let nt_header = (dos_header as usize + (*dos_header).e_lfanew as usize) as *mut IMAGE_NT_HEADERS64;
        if (*nt_header).Signature != IMAGE_NT_SIGNATURE {
            panic!("[!] INVALID NT SIGNATURE");
        }

        println!("==================== FILE HEADER ==========================");
        let file_header = (*nt_header).FileHeader;
        println!("[+] (FILE_HEADER) Arch: {}", if file_header.Machine == IMAGE_FILE_MACHINE_I386 { "x32" } else { "x64" });

        println!("[+] Number of sections: {}", file_header.NumberOfSections);
        println!("[+] Size Optional Header: {}\n", file_header.SizeOfOptionalHeader);

        println!("==================== OPTIONAL HEADER ======================");
        let optional_header = (*nt_header).OptionalHeader;
        if optional_header.Magic != IMAGE_NT_OPTIONAL_HDR_MAGIC {
            panic!("[!] Invalid IMAGE_NT_OPTIONAL_HDR_MAGIC");
        }

        println!("[+] (OPTIONAL_HEADER) Arch: {}", if optional_header.Magic == IMAGE_NT_OPTIONAL_HDR32_MAGIC { "x32" } else { "x64" });
        println!("[+] Section Size code: {}", optional_header.SizeOfCode);
        println!("[+] File Checksum: {}", optional_header.CheckSum);
        println!("[+] Required Version: {}.{}", optional_header.MajorOperatingSystemVersion, optional_header.MinorOperatingSystemVersion);
        println!("[+] Number of entries in the DataDirectory: {}\n", optional_header.NumberOfRvaAndSizes);

        println!("==================== DIRECTORIES ==========================");
        println!(
            "[+] EXPORT DIRECTORY WITH SIZE: {} | RVA: 0x{:08X}",
            optional_header.DataDirectory[IMAGE_DIRECTORY_ENTRY_EXPORT.0 as usize].Size,
            optional_header.DataDirectory[IMAGE_DIRECTORY_ENTRY_EXPORT.0 as usize].VirtualAddress
        );
        
        println!(
            "[+] IMPORT DIRECTORY WITH SIZE: {} | RVA: 0x{:08X}",
            optional_header.DataDirectory[IMAGE_DIRECTORY_ENTRY_IMPORT.0 as usize].Size,
            optional_header.DataDirectory[IMAGE_DIRECTORY_ENTRY_IMPORT.0 as usize].VirtualAddress
        );

        println!(
            "[+] RESOURCE DIRECTORY WITH SIZE: {} | RVA: 0x{:08X}",
            optional_header.DataDirectory[IMAGE_DIRECTORY_ENTRY_RESOURCE.0 as usize].Size,
            optional_header.DataDirectory[IMAGE_DIRECTORY_ENTRY_RESOURCE.0 as usize].VirtualAddress
        );
        
        println!(
            "[+] EXCEPTION DIRECTORY WITH SIZE: {} | (RVA: 0x{:08X})",
            optional_header.DataDirectory[IMAGE_DIRECTORY_ENTRY_EXCEPTION.0 as usize].Size,
            optional_header.DataDirectory[IMAGE_DIRECTORY_ENTRY_EXCEPTION.0 as usize]
                .VirtualAddress
        );
        
        println!(
            "[+] BASE RELOCATION TABLE WITH SIZE: {} | (RVA: 0x{:08X})",
            optional_header.DataDirectory[IMAGE_DIRECTORY_ENTRY_BASERELOC.0 as usize].Size,
            optional_header.DataDirectory[IMAGE_DIRECTORY_ENTRY_BASERELOC.0 as usize]
                .VirtualAddress
        );
        
        println!(
            "[+] TLS DIRECTORY WITH SIZE: {} | (RVA: 0x{:08X})",
            optional_header.DataDirectory[IMAGE_DIRECTORY_ENTRY_TLS.0 as usize].Size,
            optional_header.DataDirectory[IMAGE_DIRECTORY_ENTRY_TLS.0 as usize].VirtualAddress
        );
        
        println!(
            "[+] IMPORT ADDRESS TABLE WITH SIZE: {} | (RVA: 0x{:08X})\n",
            optional_header.DataDirectory[IMAGE_DIRECTORY_ENTRY_IAT.0 as usize].Size,
            optional_header.DataDirectory[IMAGE_DIRECTORY_ENTRY_IAT.0 as usize].VirtualAddress
        );
        
        println!("==================== SECTIONS =============================");
        let mut section_header = (nt_header as usize + size_of::<IMAGE_NT_HEADERS64>())
            as *mut IMAGE_SECTION_HEADER;

        for _ in 0..file_header.NumberOfSections {
            println!("[#] {}", std::str::from_utf8(&(*section_header).Name)?);
            println!("\tSize: {}", (*section_header).SizeOfRawData);
            println!("\tRVA: 0x{:08X}", (*section_header).VirtualAddress);
            println!("\tRelocations: {}", (*section_header).NumberOfRelocations);
            println!("\tAddress: 0x{:016X}", buffer.as_ptr() as usize + (*section_header).VirtualAddress as usize);
            
            println!("\tPermissions: ");
            if (*section_header).Characteristics & IMAGE_SCN_MEM_READ != IMAGE_SECTION_CHARACTERISTICS(0)
            {
                println!("\t\tPAGE_READONLY")
            }
            
            if (*section_header).Characteristics & IMAGE_SCN_MEM_WRITE != IMAGE_SECTION_CHARACTERISTICS(0)
            {
                println!("\t\tPAGE_READWRITE")
            }
            
            if (*section_header).Characteristics & IMAGE_SCN_MEM_EXECUTE != IMAGE_SECTION_CHARACTERISTICS(0) 
            {
                println!("\t\tPAGE_EXECUTE")
            }
            
            if (*section_header).Characteristics & IMAGE_SCN_MEM_EXECUTE != IMAGE_SECTION_CHARACTERISTICS(0)
                && (*section_header).Characteristics & IMAGE_SCN_MEM_READ != IMAGE_SECTION_CHARACTERISTICS(0)
            {
                println!("\t\tPAGE_EXECUTE_READWRITE")
            }
            
            section_header = section_header.add(1);
        }
    }

    Ok(())
}
