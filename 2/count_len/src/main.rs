fn count_len(text: &String) -> usize {// ✅ รับเป็น reference
    // text.push_str("!!!"); // ❌ ไม่สามารถเปลี่ยนแปลงค่าได้
    // text = &String::from("Hello"); // ❌ ไม่สามารถเปลี่ยนแปลงค่าได้
    text.len() // ✅ ยืมแบบ & → เรียก method ได้เลย
}

fn main() {
    let s = String::from("RustLang");
    println!("Length: {}", count_len(&s)); // ✅ ส่งเป็น reference
    println!("Original string: {}", s);    // ✅ ยังใช้ได้ เพราะไม่ถูก move
}
