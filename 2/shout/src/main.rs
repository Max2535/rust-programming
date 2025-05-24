fn shout(s: &mut String) {// ✅ รับเป็น mutable reference
     //s = &String::from("Hello"); // ❌ ไม่สามารถเปลี่ยนแปลงค่าได้
    s.push_str("!!!"); // ✅ ใช้ mutable reference เพื่อเปลี่ยนค่า
}

fn main() {
    let mut message = String::from("hello");
    shout(&mut message); // ✅ borrow แบบแก้ไข
    println!("{}", message); // ผลลัพธ์: hello!!!
}
