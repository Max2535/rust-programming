fn average(numbers: &Vec<i32>) -> f64 {
    if numbers.is_empty() {
        return 0.0;
    }

    let sum: i32 = numbers.iter().sum();
    let count = numbers.len() as f64;

    sum as f64 / count
}

fn main() {
    let nums = vec![10, 20, 30, 40];
    let avg = average(&nums);

    println!("Average = {:.2}", avg);
}
