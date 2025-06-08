use std::cell::RefCell;
fn main() {
    let try_result = std::panic::catch_unwind(|| {
        let data = RefCell::new(10);
        let _r = data.borrow();      // immutable borrow
        let _w = data.borrow_mut();  // ❌ panic here
    });

    match try_result {
        Ok(_) => println!("No panic"),
        Err(_) => println!("✅ Panic caught! Borrow conflict"),
    }
}
