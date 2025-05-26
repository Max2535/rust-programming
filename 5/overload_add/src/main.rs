use std::ops::Add;

#[derive(Debug, Copy, Clone)]
struct MyVal(u32);

// implement trait Add for MyVal
impl Add for MyVal {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        MyVal(self.0 + rhs.0)
    }
}

fn main() {
    let a = MyVal(10);
    let b = MyVal(5);
    let result = a + b; // ✅ ใช้ + ได้
    println!("{:?}", result); // Output: MyVal(15)
}
