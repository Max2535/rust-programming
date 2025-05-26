fn parse_number(s: &str) -> Result<i32, String> {
    let n = s.parse::<i32>().map_err(|_| "Invalid number".to_string())?;
    Ok(n)
}

fn main() {
    match parse_number("42") {
        Ok(n) => println!("Parsed number: {}", n),     // ✅ 42
        Err(e) => println!("Error: {}", e),
    }

    match parse_number("hello") {
        Ok(n) => println!("Parsed number: {}", n),
        Err(e) => println!("Error: {}", e),            // ❌ "Invalid number"
    }
}
