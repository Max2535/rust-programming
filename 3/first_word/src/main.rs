fn first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes(); // แปลงเป็น byte slice

    for (i, &b) in bytes.iter().enumerate() {
        if b == b' ' {
            return &s[..i]; // คืน slice ก่อนช่องว่าง
        }
    }

    &s[..] // ถ้าไม่มีช่องว่าง → คืนทั้งหมด
}

fn main() {
    let sentence = String::from("Rust is fast");
    let word = first_word(&sentence);
    println!("First word: {}", word); // "Rust"
}
