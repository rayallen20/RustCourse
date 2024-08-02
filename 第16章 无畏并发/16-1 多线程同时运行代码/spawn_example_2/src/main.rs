use std::thread;
use std::time::Duration;

fn main() {
    let join_handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(200));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(200));
    }

    // join点
    join_handle.join().unwrap();
}
