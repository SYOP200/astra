pub fn parse(input: &str) -> Vec<String> {
    input
        .split_whitespace()
        .map(String::from)
        .collect()
}
