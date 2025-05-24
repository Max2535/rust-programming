fn get_first(s: &str) -> &str {
    // แปลงเป็น byte เพื่อเช็คช่องว่าง
    let bytes = s.as_bytes();

    for (i, &b) in bytes.iter().enumerate() {
        if b == b' ' {
            return &s[0..i]; // คืน slice ก่อนช่องว่าง
        }
    }

     &s[..] // ถ้าไม่มีช่องว่าง → คืนทั้งหมด
}

fn main() {
    let s = String::from("hello world");
    let word = get_first(&s);
    println!("First word: {}", word); // "hello"
}
