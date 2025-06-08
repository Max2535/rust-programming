fn product_all(nums: &[i32]) -> i32 {
    nums.iter().fold(1, |acc, x| acc * x)
}
fn main() {
    let numbers = vec![2, 3, 4];
    let result = product_all(&numbers);
    println!("Product: {}", result); // ✅ 24
}
