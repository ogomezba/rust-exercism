pub fn reverse(input: &str) -> String {
    let chars: Vec<char> = input.chars().collect();
    let result = reverse_recursive(&chars);

    result.iter().collect()
}

fn reverse_recursive(char_vec: &[char]) -> Vec<char> {
    if char_vec.is_empty() {
        Vec::new()
    } else {
        let (head, tail) = char_vec.split_first().unwrap();
        let mut new_vec = reverse_recursive(tail);
        new_vec.push(*head);

        new_vec
    }
}
