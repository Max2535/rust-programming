fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

fn main() {
    let x = String::from("Rust");
    let y = String::from("Ownership");

    let result = longest(&x, &y);
    println!("Longest: {}", result); // Output: Ownership
}
