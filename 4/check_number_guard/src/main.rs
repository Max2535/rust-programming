fn check_number_guard(n: u8) {
    match n {
        1..=10 if n % 2 == 0 => println!("Even"),
        1..=10 => println!("Odd"),
        _ => println!("Out of range"),
    }
}

fn main() {
    check_number_guard(3);  // Odd
    check_number_guard(8);  // Even
    check_number_guard(0);  // Out of range
    check_number_guard(11); // Out of range
}
