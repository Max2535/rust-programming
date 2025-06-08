use rayon::prelude::*;

fn filter_sum(nums: &[i32]) -> i32 {
    nums.par_iter()
        .copied()
        .filter(|x| x % 3 == 0)
        .sum()
}
fn main() {
    let numbers = vec![3, 5, 6, 8, 9, 10];
    let result = filter_sum(&numbers);
    println!("Sum = {}", result); // ✅ 3 + 6 + 9 = 18
}
