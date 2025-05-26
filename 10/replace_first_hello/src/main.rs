fn replace_first_hello(input: &str) -> String {
    input.replacen("hello", "hi", 1)
}
fn main() {
    let text: &'static str = "hello world, hello again, hello!";
    let result = replace_first_hello(text);
    println!("{}", result); // hi world, hello again, hello!
}
