const VALUE: &str = "Ready";

fn get_status() -> &'static str {
    //"OK" // string literal มี lifetime เป็น 'static โดยอัตโนมัติ
    VALUE
}

fn main() {
    let status = get_status();
    println!("Status: {}", status); // Output: Status: OK
}
