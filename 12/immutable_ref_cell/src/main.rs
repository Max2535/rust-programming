use std::cell::RefCell;

struct Person {
    name: RefCell<String>,
}

fn main() {
    let person = Person {
        name: RefCell::new(String::from("Alice")),
    };

    // เปลี่ยนชื่อแม้ person เป็น immutable
    *person.name.borrow_mut() = String::from("Bob");

    println!("Name = {}", person.name.borrow()); // ✅ Bob
}
