macro_rules! vec_of_strings {
    ( $( $x:expr ),* ) => {
        vec![$($x.to_string()),*]
    };
}
fn main() {
    let names = vec_of_strings!["Alice", "Bob", "Carol"];
    println!("{:?}", names); // ✅ ["Alice", "Bob", "Carol"]
}
