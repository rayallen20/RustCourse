use std::sync::Mutex;
use std::thread;
use std::thread::JoinHandle;

fn main() {
    let counter = Mutex::new(0);
    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    // fork点
    for _ in 1..10 {
        // 这里的错误在于 当第1个线程获取了锁的所有权之后 循环中的后续线程就无法再获取锁的所有权了
        let handle = thread::spawn(move || { // error: value moved into closure here, in previous iteration of loop
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    // join点
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
