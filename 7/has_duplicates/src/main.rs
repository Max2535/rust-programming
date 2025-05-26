use std::collections::HashSet;

fn has_duplicates(nums: Vec<i32>) -> bool {
    let mut seen = HashSet::new();

    for n in nums {
        if !seen.insert(n) {
            return true; // insert() คืน false ถ้ามีค่าอยู่แล้ว
        }
    }

    false
}

fn main() {
    println!("{}", has_duplicates(vec![1, 2, 3, 4])); // false
    println!("{}", has_duplicates(vec![1, 2, 2, 3])); // true
    println!("{}", has_duplicates(vec![]));          // false
}
