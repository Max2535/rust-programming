use std::fs::File;
use std::io::{self, Read};

fn read_file(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;             // ถ้าเปิดไม่ได้ → return Err
    let mut content = String::new();
    file.read_to_string(&mut content)?;           // ถ้าอ่านไม่ได้ → return Err
    Ok(content)
}

fn main() {
    match read_file("data.txt") {
        Ok(text) => println!("File contents:\n{}", text),
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}
