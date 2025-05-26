use std::ops::Mul;

#[derive(Debug, Copy, Clone)]
struct Vec2(i32, i32);

// implement * operator
impl Mul for Vec2 {
    type Output = i32;

    fn mul(self, rhs: Self) -> i32 {
        self.0 * rhs.0 + self.1 * rhs.1
    }
}

fn main() {
    let a = Vec2(2, 3);
    let b = Vec2(4, 5);
    let result = a * b; // 2*4 + 3*5 = 8 + 15 = 23
    println!("Dot product = {}", result);
}
