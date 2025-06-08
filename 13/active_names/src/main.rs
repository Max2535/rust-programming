use rayon::prelude::*;

#[derive(Debug)]
struct User {
    name: String,
    active: bool,
}

fn active_names(users: &[User]) -> Vec<String> {
    users.par_iter()
        .filter(|u| u.active)
        .map(|u| u.name.clone())
        .collect()
}
fn main() {
    let users = vec![
        User { name: "Alice".into(), active: true },
        User { name: "Bob".into(), active: false },
        User { name: "Carol".into(), active: true },
    ];

    let result = active_names(&users);
    println!("{:?}", result); // ✅ ["Alice", "Carol"]
}
