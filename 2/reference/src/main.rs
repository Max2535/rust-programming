fn main() {
    let mut s = String::from("borrow");

    let r1 = &s;
    let r2 = &s;
    //let r3 = &mut s;
    println!("{}, {}", r1, r2); // ✅ ใช้ reference แบบอ่านให้จบก่อน
    let r3 = &mut s;
    println!("{}", r3); // ✅ OK
}
