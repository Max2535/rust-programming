fn swap(a: &mut i32, b: &mut i32) { // ฟังก์ชัน swap รับ mutable reference ของ i32 สองตัว
    // ใช้การ dereference เพื่อเข้าถึงค่าใน reference
    // และทำการสลับค่าระหว่าง a และ b
    let temp = *a;
    *a = *b;
    *b = temp;
}

fn main() {
    let mut x = 10;
    let mut y = 20;

    println!("Before: x = {}, y = {}", x, y);
    // เรียกใช้ฟังก์ชัน swap โดยส่ง mutable reference ของ x และ y
    swap(&mut x, &mut y);//
    println!("After: x = {}, y = {}", x, y);
}
