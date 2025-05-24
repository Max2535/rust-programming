fn level(score: u8) -> &'static str {
    match score {
        90..=100 => "Excellent",
        70..=89 => "Good",
        0..=69 => "Fail",
        _ => "Invalid",
    }
}

fn main() {
    println!("{}", level(99)); // Excellent
    println!("{}", level(75)); // Good
    println!("{}", level(60)); // Fail
    println!("{}", level(150)); // Invalid
}
