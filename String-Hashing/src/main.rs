fn main() {
    let message_box = "MessageBoxA";

    dbj2(message_box);
    jenkins_one_at_atime32_bit(message_box);
    lose_lose(message_box);
    rotr32(message_box);
}

// https://github.com/vxunderground/VX-API/blob/main/VX-API/HashStringDjb2.cpp
fn dbj2(string: &str) {
    let mut hash: u32 = 5381;

    for c in string.bytes() {
        hash = ((hash << 5).wrapping_add(hash)).wrapping_add(c as u32);
    }

    println!("Hash using dbj2 from the string {} is: 0x{:08X}", string, hash);
}

// https://github.com/vxunderground/VX-API/blob/main/VX-API/HashStringJenkinsOneAtATime32Bit.cpp
fn jenkins_one_at_atime32_bit(string: &str) {
    let mut hash = 0u32;

    for c in string.bytes() {
        hash = hash.wrapping_add(c as u32);
        hash = hash.wrapping_add(hash << 10);
        hash ^= hash >> 6;
    }

    hash = hash.wrapping_add(hash << 3);
    hash ^= hash >> 11;
    hash = hash.wrapping_add(hash << 15);

    println!("Hash using JenkinsOneAtATime32Bit from the string {} is: 0x{:08X}", string, hash);
}

// https://github.com/vxunderground/VX-API/blob/main/VX-API/HashStringLoseLose.cpp
fn lose_lose(string: &str) {
    let mut hash = 0u32;

    for c in string.bytes() {
        hash = hash.wrapping_add(c as u32);
        hash = hash.wrapping_mul(c as u32 + 2);
    }

    println!("Hash using LoseLose from the string {} is: 0x{:08X}", string, hash);
}

// https://github.com/vxunderground/VX-API/blob/main/VX-API/HashStringRotr32.cpp#L3
fn rotr32_sub(value: u32, count: u32) -> u32 {
    let mask = 8 * std::mem::size_of::<u32>() as u32 - 1;
    let count = count & mask;
    (value >> count) | (value << (mask + 1 - count))
}

// https://github.com/vxunderground/VX-API/blob/main/VX-API/HashStringRotr32.cpp#L13
fn rotr32(string: &str) {
    let mut value = 0;

    for &c in string.as_bytes() {
        value = c as u32 + rotr32_sub(value, 7);
    }

    println!("Hash using Rotr32 from the string {} is: 0x{:08X}", string, value);
}
