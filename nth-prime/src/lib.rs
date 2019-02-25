pub fn nth(n: u32) -> u32 {
    let mut prime_count = 1;
    let mut x = 2;
    while prime_count <= n {
        x += 1;
        if is_prime(x) {
            prime_count += 1;
        }
    }
    return x;
}

pub fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    } else if n == 2 {
        return true;
    }
    let sqrt = (n as f64).sqrt() as u32;
    for x in 2..=sqrt {
        if n % x == 0 {
            return false;
        }
    }
    return true;
}
