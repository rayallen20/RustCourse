use std::sync::Mutex;
use std::thread;
use std::thread::JoinHandle;

fn main() {
    let counter = Mutex::new(0);
    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    // fork点1
    let handle1 = thread::spawn(move || {
        let mut num = counter.lock().unwrap();
        *num += 1;
    });
    handles.push(handle1);

    // fork点2
    let handle2 = thread::spawn(move || { // error: value moved into closure here, in previous iteration of loop
        let mut num = counter.lock().unwrap();
        *num += 1;
    });
    handles.push(handle2);

    // join点
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap()); // error: value borrowed here after move
}
