pub fn square(s: u32) -> u64 {
    if s < 1 || s > 64 {
        panic!("Square must be between 1 and 64")
    }
    return 1u64.rotate_left(s - 1);
}

pub fn total() -> u64 {
    return std::u64::MAX;
}
