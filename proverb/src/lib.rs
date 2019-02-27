pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new();
    }
    let mut result = String::new();
    for i in 1..list.len() {
        let intermediate_string = format!("For want of a {} the {} was lost.", list[i - 1], list[i]);
        result.push_str(&intermediate_string);
        result.push('\n');
    }
    let first_item = list[0];
    let last_statement = format!("And all for the want of a {}.", first_item);
    result.push_str(&last_statement);
    return result;
}
