use std::io;

fn main() {
    let mut input_a = String::new();
    let mut input_b = String::new();

    println!("Enter value for a:");
    io::stdin().read_line(&mut input_a).expect("Failed to read a");

    println!("Enter value for b:");
    io::stdin().read_line(&mut input_b).expect("Failed to read b");

    let mut a: i32 = input_a.trim().parse().expect("Invalid number for a");
    let mut b: i32 = input_b.trim().parse().expect("Invalid number for b");

    println!("Before swap: a = {}, b = {}", a, b);

    // ✅ สลับค่า
    let temp = a;
    a = b;
    b = temp;

    println!("After swap: a = {}, b = {}", a, b);
}
