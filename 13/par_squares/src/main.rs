use rayon::prelude::*; // สำคัญมาก ต้อง import trait!

fn par_squares(nums: &[i32]) -> Vec<i32> {
    nums.par_iter()
        .map(|x| x * x)
        .collect()
}

fn main() {
    let input = vec![1, 2, 3, 4, 5];
    let result = par_squares(&input);
    println!("{:?}", result); // ✅ เช่น [1, 4, 9, 16, 25]
}
