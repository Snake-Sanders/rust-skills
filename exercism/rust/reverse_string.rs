pub fn reverse(input: &str) -> String {
    let mut s = String::new();
    for c in input.chars().rev() {
        s.push(c)
    }

    return s;
}
