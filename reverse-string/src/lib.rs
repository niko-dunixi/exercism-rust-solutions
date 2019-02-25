pub fn reverse(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    for character in input.chars().rev() {
        result.push(character);
    }
    return result;
}
