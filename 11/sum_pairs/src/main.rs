fn sum_pairs(a: &[i32], b: &[i32]) -> Vec<i32> {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| x + y)
        .collect()
}

fn main() {
    let a = vec![1, 2, 3];
    let b = vec![4, 5, 6];
    let result = sum_pairs(&a, &b);
    println!("{:?}", result); // ✅ [5, 7, 9]
}
