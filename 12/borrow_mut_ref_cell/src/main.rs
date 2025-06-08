use std::cell::RefCell;

fn main() {
    let counter: RefCell<i32> = RefCell::new(0); // เริ่มจาก 0

    *counter.borrow_mut() += 1;
    *counter.borrow_mut() += 1;
    println!("Counter = {}", counter.borrow()); // ✅ 2

    // Show the address of the RefCell itself
    println!("Address of RefCell: {:p}", &counter);

    // Show the address of the inner value
    println!("Address of inner value: {:p}", counter.as_ptr());

    println!("Counter = {}", counter.borrow()); 
}
