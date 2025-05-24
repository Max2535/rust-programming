fn smart_check(n: u8) {
    match n {
        1 | 2 | 3 | 4 | 5 if n % 2 == 0 => println!("Even between 1-5"),
        1..=5 => println!("Odd between 1-5"),
        _ => println!("Out of range"),
    }
}

fn main() {
    smart_check(2);  // Even between 1-5
    smart_check(3);  // Odd between 1-5
    smart_check(0);  // Out of range
    smart_check(6);  // Out of range
}
