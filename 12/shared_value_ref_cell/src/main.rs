use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let shared_value = Rc::new(RefCell::new(5));

    let a = Rc::clone(&shared_value);
    let b = Rc::clone(&shared_value);

    *a.borrow_mut() += 10;   // 5 → 15
    *b.borrow_mut() *= 2;    // 15 × 2 = 30

    println!("Final value = {}", shared_value.borrow()); // ✅ 30
}
