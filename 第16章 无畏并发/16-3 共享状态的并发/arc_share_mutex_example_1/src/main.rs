use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    // fork点
    for _ in 1..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // 这里由于Arc<T>也是一个智能指针,实现了Deref Trait,因此它可以自动解引用
            // 如果没有实现Deref Trait,则需要使用(*counter).lock().unwrap()来解引用
            // 代码如下
            // let mut num = (*counter).lock().unwrap();
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
