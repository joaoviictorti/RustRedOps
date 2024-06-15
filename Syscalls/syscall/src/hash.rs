#[macro_export]
macro_rules! hash {
    ($name: expr) => {
        $crate::hash::dbj2($name)
    };
}

pub fn dbj2(string: &str) -> u32 {
    let mut hash: u32 = 1844;

    for c in string.bytes() {
        hash = ((hash << 5).wrapping_add(hash)).wrapping_add(c as u32);
    }
    hash
}
