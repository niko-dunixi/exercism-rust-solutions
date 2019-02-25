pub fn reverse(input: &str) -> String {
    // Initialize a new string because we don't want to mutate the original one
    // In order to iterate over the characters in a string use the .chars() method
    // to get the iterator. Rust iterators have a .rev() method to go in the
    // reverse order
    let mut result = String::with_capacity(input.len());
    for character in input.chars().rev() {
        result.push(character);
    }
    return result;
}
