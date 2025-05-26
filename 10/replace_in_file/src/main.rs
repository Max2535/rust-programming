use std::fs;
use std::io;

fn replace_in_file() -> io::Result<()> {
    // 1. อ่านไฟล์ต้นฉบับ
    let content = fs::read_to_string("input.txt")?;

    // 2. แทนคำ
    let replaced = content.replace("Output", "");

    // 3. เขียนผลลัพธ์ลงไฟล์ใหม่
    fs::write("output.txt", replaced)?;

    println!("✅ Finished writing to output.txt");
    Ok(())
}

fn main() {
    if let Err(e) = replace_in_file() {
        eprintln!("❌ Error: {}", e);
    }
}
