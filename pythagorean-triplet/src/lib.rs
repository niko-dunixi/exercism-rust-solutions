use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    // Let's break this down. sum = a + b + c
    // We need to get m and n in order to find the triples (a, b, c).
    // The triples take the form of (m^2 - n^2, 2mn, m^2 + n^2)
    // We can start solving taking advantage of this equality
    //  a + b + c = 2m(m + n)
    unimplemented!("Given the sum {}, return all possible Pythagorean triplets, which produce the said sum, or an empty HashSet if there are no such triplets. Note that you are expected to return triplets in [a, b, c] order, where a < b < c", sum);
}

// References:
// 1] https://www.quora.com/How-do-I-generate-Pythagorean-triples-given-the-sum-of-the-triples
// 2] https://www.youtube.com/watch?v=B2FLARYs3bo
