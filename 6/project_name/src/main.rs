// ประกาศ mod และนำเข้า module
mod utils {
    pub mod math;
}

use utils::math::{add, mul};

fn main() {
    let x = add(2, 3);
    let y = mul(4, 5);
    println!("Add: {}, Mul: {}", x, y); // Add: 5, Mul: 20
}
