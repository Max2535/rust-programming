fn main() {
    let nums = [9, 2, 5, 8, 1, 4];

    // สมมุติให้ค่ามากสุดเริ่มที่ตัวแรก
    let mut max = nums[0];

    for &num in nums.iter() {
        if num > max {
            max = num;
        }
    }

    println!("Maximum value is: {}", max);
}
