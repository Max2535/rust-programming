macro_rules! repeat {
    ($val:expr, $count:expr) => {
        vec![$val; $count]
    };
}
fn main() {
    let repeated = repeat!(42, 5);
    println!("{:?}", repeated); // ✅ [42, 42, 42, 42, 42]

    let hello = repeat!("hi".to_string(), 3);
    println!("{:?}", hello); // ✅ ["hi", "hi", "hi"]
}
