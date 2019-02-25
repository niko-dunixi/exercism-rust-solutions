pub fn factors(n: u64) -> Vec<u64> {
    let mut factor_results = Vec::new();
    // We need to track the current factor and also our changing n.
    // So we test our current factor, if n is divisible by our current factor,
    // we store our current factor and divide n by that factor. Rinse and repeat
    // until the factor no longer divides current n. We then increment our factor
    // that we're testing and start the process over.
    let mut current_factor = 2;
    let mut current_n = n;
    while current_factor <= current_n {
        if current_n % current_factor == 0 {
            factor_results.push(current_factor);
            current_n = current_n / current_factor;
        } else {
            current_factor += 1;
        }
    }
    return factor_results;
}
