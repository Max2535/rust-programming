use regex::Regex;

fn wrap_ids(input: &str) -> String {
    let re = Regex::new(r"ID: (\d+)").unwrap();
    re.replace_all(input, |caps: &regex::Captures| {
        format!("ID: [{}]", &caps[1])
    }).to_string()
}

fn mask_phone(input: &str) -> String {
    let re = Regex::new(r"^(\d{3})(\d{3})(\d{4})$").unwrap();

    if let Some(caps) = re.captures(input) {
        format!("{}-{}x-xxx", &caps[1], &caps[2])
    } else {
        "Invalid phone number".to_string()
    }
}

fn main() {
    // let text = "User A has ID: 42, User B has ID: 99.";
    // let result = wrap_ids(text);
    // println!("{}", result);
    let phone = "0987654321";
    let masked = mask_phone(phone);
    println!("{}", masked); // ✅ 098-765x-xxx
}
