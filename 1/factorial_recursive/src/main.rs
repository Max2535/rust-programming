fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn main() {
    let input = 5;
    println!("Factorial of {} = {}", input, factorial(input));
}
