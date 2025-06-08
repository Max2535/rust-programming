use rayon::prelude::*;

fn sorted(mut nums: Vec<i32>) -> Vec<i32> {
    nums.par_sort(); // ✅ ขนานและเรียงลำดับ
    nums
}

fn main() {
    let nums = vec![5, 3, 8, 6, 2, 7, 4, 1];
    let sorted_nums = sorted(nums);
    println!("{:?}", sorted_nums); // ✅ แสดงผลลัพธ์ที่เรียงลำดับแล้ว
}