enum Status {
    Idle,
    Loading,
    Success,
    Failed,
}

fn report(status: Status) {
    match status {
        Status::Success | Status::Failed => println!("Done."),
        Status::Loading => println!("Working..."),
        Status::Idle => println!("Waiting..."),
    }
}

fn main() {
    report(Status::Idle);     // Waiting...
    report(Status::Loading);  // Working...
    report(Status::Success);  // Done.
    report(Status::Failed);   // Done.
}
