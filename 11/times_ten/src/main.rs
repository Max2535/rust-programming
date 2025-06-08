fn times_ten() -> Vec<i32> {
    (1..=10)
        .map(|x| x * 10)
        .collect()
}
fn main() {
    let result = times_ten();
    println!("{:?}", result); // ✅ [10, 20, 30, 40, 50, 60, 70, 80, 90, 100]
}
