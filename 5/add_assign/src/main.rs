use std::ops::AddAssign;

#[derive(Debug)]
struct Counter(u32);

// implement += operator
impl AddAssign<u32> for Counter {
    fn add_assign(&mut self, rhs: u32) {
        self.0 += rhs;
    }
}

fn main() {
    let mut count = Counter(0);
    count += 1;
    count += 5;
    println!("{:?}", count); // Output: Counter(6)
}
