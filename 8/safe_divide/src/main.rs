fn safe_divide(a: &str, b: &str) -> Result<i32, String> {
    let x: i32 = a.parse().map_err(|_| "Invalid number for a")?;
    let y: i32 = b.parse().map_err(|_| "Invalid number for b")?;

    if y == 0 {
        return Err("Division by zero".to_string());
    }

    Ok(x / y)
}
fn main() {
    let cases: [(&'static str, &'static str); 3] = [("10", "2"), ("5", "0"), ("abc", "1")];

    for (a, b) in cases {
        match safe_divide(a, b) {
            Ok(res) => println!("{}/{} = {}", a, b, res),
            Err(e) => println!("Error dividing {} / {}: {}", a, b, e),
        }
    }
}
