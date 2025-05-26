use std::ops::Add;

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

// implement Add<i32> for Point
impl Add<i32> for Point {
    type Output = Self;

    fn add(self, rhs: i32) -> Self {
        Point {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

fn main() {
    let p = Point { x: 1, y: 2 };
    let p2 = p + 5;
    println!("{:?}", p2); // Point { x: 6, y: 7 }
}
