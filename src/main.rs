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

    println!("Hello, World1!");
    println!("Hello, World2!");
    println!("Hello, World3!");
    println!("Hello, World4!");
    println!("Hello, World5!");
    println!("Hello, World6!");
    println!("Hello, World7!");
    println!("Hello, World8!");

    println!("{}", t);

    t.complete();
    println!("After completion: {}", t);
}
