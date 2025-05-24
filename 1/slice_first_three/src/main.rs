fn first_three(arr: &[i32]) -> &[i32] {
    let len = arr.len();

    if len >= 3 {
        &arr[..3]
    } else {
        &arr[..]
    }
}

fn main() {
    let data1 = [1, 2, 3, 4, 5];
    let data2 = [9, 8];

    let slice1 = first_three(&data1);
    let slice2 = first_three(&data2);

    println!("First three (data1): {:?}", slice1);
    println!("First three (data2): {:?}", slice2);
}
// Output:
// First three (data1): [1, 2, 3]
// First three (data2): [9, 8]
//