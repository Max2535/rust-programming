#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}


// // implement == operator
// impl PartialEq for Point {
//     fn eq(&self, other: &Self) -> bool {
//         self.x == other.x && self.y == other.y
//     }
// }

fn main() {
    let p1 = Point { x: 3, y: 4 };
    let p2 = Point { x: 3, y: 4 };
    let p3 = Point { x: 5, y: 6 };

    println!("p1 == p2: {}", p1 == p2); // true
    println!("p1 == p3: {}", p1 == p3); // false
    println!("p1 != p3: {}", p1 != p3); // true
}
