use std::rc::Rc;
use std::sync::Mutex;
use std::thread;
use std::thread::JoinHandle;

fn main() {
    let counter = Rc::new(Mutex::new(0));
    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    // fork点
    for _ in 1..10 {
        let counter = Rc::clone(&counter);
        let handle = thread::spawn(move || {  // `Rc<Mutex<i32>>` cannot be sent between threads safely
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
