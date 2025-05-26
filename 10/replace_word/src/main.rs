fn replace_word(input: &str, from: &str, to: &str) -> String {
    input.replace(from, to)
}
fn main() {
    let input = "rust is fast. rust is fun.";
    let output = replace_word(input, "rust", "go");

    println!("{}", output); // go is fast. go is fun.
}
