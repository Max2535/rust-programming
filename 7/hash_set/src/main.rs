use std::collections::HashSet;

fn main() {
    let a: HashSet<_> = [1, 2, 3, 4].iter().cloned().collect();
    let b: HashSet<_> = [3, 4, 5, 6].iter().cloned().collect();

    let intersection: HashSet<_> = a.intersection(&b).cloned().collect();
    let difference: HashSet<_> = a.difference(&b).cloned().collect();
    let union: HashSet<_> = a.union(&b).cloned().collect();

    println!("A = {:?}", a);  
    println!("B = {:?}", b);  
    println!("Intersection (A ∩ B) = {:?}", intersection);  // {3, 4}
    println!("Difference (A - B) = {:?}", difference);      // {1, 2}
    println!("Union (A ∪ B) = {:?}", union);                // {1, 2, 3, 4, 5, 6}
}
