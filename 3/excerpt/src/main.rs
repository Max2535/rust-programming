struct Excerpt<'a> {
    part: &'a str,
}

impl<'a> Excerpt<'a> {
    fn announce(&self) {
        println!("Excerpt: {}", self.part);
    }
}

fn main() {
    let text = String::from("Rust is safe and fast.");
    let first_part = &text[0..4]; // "Rust"

    let excerpt = Excerpt { part: first_part };
    excerpt.announce(); // Output: Excerpt: Rust
}
