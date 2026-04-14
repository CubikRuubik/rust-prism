mod task;

use std::time::{Duration, SystemTime};
use task::{Priority, Task};

fn main() {
    let due = SystemTime::now() + Duration::from_secs(48 * 3600);
    let mut t = Task::new(
        "Implement prism core",
        "Build the main refraction logic for rust-prism",
        Priority::High,
        due,
    );

    println!("{}", t);

    t.complete();
    println!("After completion: {}", t);
}
