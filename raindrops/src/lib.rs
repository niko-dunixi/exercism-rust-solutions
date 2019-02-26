use std::collections::HashSet;

pub fn raindrops(n: u32) -> String {
    unimplemented!("what sound does Raindrop #{} make?", n)
}

pub fn determine_factors(n: u32) -> HashSet<u32> {
    let mut set = HashSet::new();
    for value in 1..=n {
        if n % value == 0 {
            set.insert(value);
        }
    }
    return set;
}