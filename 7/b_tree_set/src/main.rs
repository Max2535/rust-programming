use std::collections::BTreeSet;

fn main() {
    let mut set = BTreeSet::new();

    set.insert(42);
    set.insert(10);
    set.insert(99);
    set.insert(42); // ซ้ำ → จะไม่ถูกเพิ่ม

    println!("เรียงจากน้อยไปมาก:");
    for val in &set {
        println!("{}", val);
    }
}
