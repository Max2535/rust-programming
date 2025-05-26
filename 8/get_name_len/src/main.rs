fn get_name_len(opt: Option<&str>) -> Option<usize> {
    Some(opt?.len()) // Using the `?` operator to handle None case
}

fn main() {
    let name1 = Some("Alice");
    let name2: Option<&str> = None;

    println!("Len 1: {:?}", get_name_len(name1)); // Some(5)
    println!("Len 2: {:?}", get_name_len(name2)); // None
}
