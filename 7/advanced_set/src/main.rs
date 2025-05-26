use std::collections::HashSet;

#[derive(Debug, Hash, Eq, PartialEq)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let mut people = HashSet::new();

    people.insert(Person { name: "Alice".into(), age: 30 });
    people.insert(Person { name: "Bob".into(), age: 25 });
    people.insert(Person { name: "Alice".into(), age: 30 }); // ❌ ซ้ำ จะไม่ถูกเพิ่ม

    for person in &people {
        println!("{:?}", person);
    }
}
