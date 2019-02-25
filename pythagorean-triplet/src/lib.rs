use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    // Let's break this down. sum = a + b + c
    // We need to get m and n in order to find the triples (a, b, c).
    // The triples take the form of (m^2 - n^2, 2mn, m^2 + n^2)
    // We can start solving taking advantage of this equality
    //  a + b + c = 2m(m + n)

    // For now. Let's do the stupid simple thing. I just want
    // to understand Rust for the moment.
    // let's implement the naive answer, but I think we can do this
    // mathematically

    let mut set: HashSet<[u32; 3]> = HashSet::with_capacity(3);
    for a in 1..=sum / 3 {
        for b in a + 1..=sum / 2 {
            let c = sum - a - b;
            if a.pow(2) + b.pow(2) == c.pow(2) {
                let result: [u32; 3] = [a, b, c];
                set.insert(result);
            }
        }
    }
    return set;
}

// References:
// 1] https://www.quora.com/How-do-I-generate-Pythagorean-triples-given-the-sum-of-the-triples
// 2] https://www.youtube.com/watch?v=B2FLARYs3bo
// 3] https://www.geeksforgeeks.org/pythagorean-triplet-given-sum/
