// It may be that at some point, as the code grows and you work with other APIs, this part of the code becomes necessary. Only use it when the compiler actually reports an error.

#[no_mangle]
pub extern "C" fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    for i in 0..n {
        unsafe { *s.add(i) = c as u8 };
    }
    s
}

#[no_mangle]
pub extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    for i in 0..n {
        unsafe {
            *dest.add(i) = *src.add(i);
        }
    }
    dest
}

#[no_mangle]
pub extern "C" fn memmove(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    if src < dest as *const u8 {
        for i in (0..n).rev() {
            unsafe {
                *dest.add(i) = *src.add(i);
            }
        }
    } else {
        for i in 0..n {
            unsafe {
                *dest.add(i) = *src.add(i);
            }
        }
    }
    dest
}

#[no_mangle]
pub extern "C" fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    for i in 0..n {
        let a = unsafe { *s1.add(i) };
        let b = unsafe { *s2.add(i) };
        if a != b {
            return a as i32 - b as i32;
        }
    }
    0
}

#[no_mangle]
pub extern "C" fn strlen(s: *const u8) -> usize {
    let mut count = 0;
    unsafe {
        while *s.add(count) != 0 {
            count += 1;
        }
    }
    count
}

#[export_name = "_fltused"]
static _FLTUSED: i32 = 0;

#[no_mangle]
pub extern "system" fn __CxxFrameHandler3(_: *mut u8, _: *mut u8, _: *mut u8, _: *mut u8) -> i32 { unimplemented!() }

#[no_mangle]
pub extern "C" fn __chkstk() {}