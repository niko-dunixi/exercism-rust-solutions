pub fn square(s: u32) -> u64 {
    // The reason this works, is because this essentially
    // just asking what value is enabled at a given binary
    // position.
    assert!(s >= 1 && s <= 64, "Square must be between 1 and 64");
    return 1u64.rotate_left(s - 1);
}

pub fn total() -> u64 {
    // The max value works because it is the same as all
    // binary positions being toggled on.
    return std::u64::MAX;
}
