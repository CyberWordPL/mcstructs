pub fn bitshift_i32_with_sign(val: i32, bitshift: u8) -> i32 {
    let u32_val = u32::from(val);
    u32_val << bitshift;
    i32::from(u32_val)
}

pub fn bitshift_i64_with_sign(val: i64, bitshift: u8) -> i64 {
    let u64_val = u64::from(val);
    u64_val << bitshift;
    i64::from(u64_val)
}