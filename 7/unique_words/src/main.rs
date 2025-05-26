use std::collections::HashSet;

fn unique_words(text: &str) -> usize {
    let words: HashSet<&str> = text.split_whitespace().collect();
    words.len()
}

fn main() {
    let sentence = "rust is fast and rust is safe";
    let count = unique_words(sentence);
    println!("Unique word count: {}", count); // Output: 5
}
