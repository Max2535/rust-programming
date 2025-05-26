fn process(name: Option<&str>, num: &str) -> Result<String, String> {
    let name = name.ok_or("no name")?; // เปลี่ยน Option → Result
    let number: i32 = num.parse().map_err(|_| "invalid number")?;

    Ok(format!("name: {}, num: {}", name, number))
}

fn main() {
    let cases = [
        (Some("Alice"), "42"),
        (Some("Bob"), "abc"),
        (None, "123"),
    ];

    for (name, num) in cases {
        match process(name, num) {
            Ok(msg) => println!("✅ {}", msg),
            Err(e) => println!("❌ Error: {}", e),
        }
    }
}
