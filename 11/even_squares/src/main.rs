fn even_squares(nums: &[i32]) -> Vec<i32> {
    nums.iter()
        .filter(|&x| x % 2 == 0)
        .map(|x| x * x)
        .collect()
}

fn main() {
    let input = vec![1, 2, 3, 4, 5];
    let output = even_squares(&input);
    println!("{:?}", output); // ✅ [4, 16]
}
