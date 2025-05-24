use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter an integer:");
    io::stdin().read_line(&mut input).expect("Failed to read");

    let num: i32 = input.trim().parse().expect("Not a valid number");

    if num % 2 == 0 {
        println!("{} is even", num);
    } else {
        println!("{} is odd", num);
    }
    /*
    match num % 2 {
    0 => println!("{} is even", num),
    1 | -1 => println!("{} is odd", num),
    _ => println!("Unexpected case"),
}
     */
}
