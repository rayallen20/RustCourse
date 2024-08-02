use std::thread;
use std::time::Duration;

fn main() {
    // 创建一个新线程
    // fork点
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(200));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(200));
    }

    // 当主线程结束时,无论子线程是否结束,整个程序都会结束
    // 这段代码中没有join点
}
