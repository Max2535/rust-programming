use rayon::prelude::*;

fn par_sum() -> i64 {
    (1..=1_000_000)
        .into_par_iter()
        .map(|x| x as i64)
        .sum()
}

fn main() {
    let total = par_sum();
    println!("Sum = {}", total); // ✅ 500000500000
}
