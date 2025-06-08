macro_rules! max {
    ($a:expr, $b:expr) => {
        (if ($a) > ($b) { ($a) } else { ($b) })
    };
}

fn main() {
    println!("Max of 4 and 7 = {}", max!(4, 7));       // ✅ 7
    println!("Max of 10 and 3 = {}", max!(10, 3));     // ✅ 10
    println!("Max of (1 + 2) and 4 = {}", max!(1 + 2, 4)); // ✅ 4
}
