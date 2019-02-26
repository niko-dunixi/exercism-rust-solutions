use std::collections::HashSet;

pub fn raindrops(n: u32) -> String {
    let mut result = String::new();
    let factors = determine_factors(n);
    if factors.contains(&3) {
        result += "Pling";
    }
    if factors.contains(&5) {
        result += "Plang";
    }
    if factors.contains(&7) {
        result += "Plong";
    }
    if result.is_empty() {
        result = n.to_string();
    }
    return result;
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