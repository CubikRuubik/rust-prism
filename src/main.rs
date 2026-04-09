mod queue;

use queue::Queue;

fn main() {
    let mut q = Queue::new();
    q.enqueue(10);
    q.enqueue(20);
    q.enqueue(30);

    println!("Queue size: {}", q.size());

    for item in q.drain() {
        println!("ID: {}, Value: {}", item.id, item.value);
    }

    println!("Queue size after drain: {}", q.size());
}
