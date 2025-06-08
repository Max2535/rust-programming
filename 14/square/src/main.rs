macro_rules! square {
    ($x:expr) => {
        ($x) * ($x)
    };
}

fn main() {
    let x = 5;
    let y = square!(x + 1);
    println!("The square of {} is {}", x + 1, y);
}
