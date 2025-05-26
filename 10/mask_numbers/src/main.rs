use regex::Regex;

fn mask_numbers(input: &str) -> String {
    let re = Regex::new(r"\d+").unwrap(); // \d+ = กลุ่มตัวเลข 1 ตัวขึ้นไป
    re.replace_all(input, "#").to_string()
}

fn main() {
    let input = "Call 12345 or 67890 now!";
    let result = mask_numbers(input);
    println!("{}", result); // Call # or # now!
}
