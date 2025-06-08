fn indexed_chars(input: &str) -> Vec<(usize, char)> {
    input.chars().enumerate().collect()
}
fn main() {
    let input = "rust";
    let result = indexed_chars(input);
    println!("{:?}", result); // ✅ [(0, 'r'), (1, 'u'), (2, 's'), (3, 't')]
}
