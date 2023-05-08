use core::ptr;

#[no_mangle]
pub unsafe extern "C" fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    for i in 0..n {
        ptr::write_volatile(s.add(i), c as u8);
    }
    s
}

pub fn strlen(s: &str) -> usize {
    s.len()
}

pub fn set_bits_with_offset(field: u32, offset: u32, width: u32, value: u32) -> u32 {
    let mask = (1 << width) - 1;
    let temp = field & !(mask << offset);
    temp | (value << offset)
}
