use std::sync::mpsc::{channel, TryRecvError};
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = channel();

    // 子线程每100ms发送一个消息
    thread::spawn(move || {
        for i in 1..=5 {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    // 主线程每50ms尝试接收一个消息
    loop {
        match rx.try_recv() {
            Ok(msg) => println!("Received: {}", msg),
            Err(TryRecvError::Empty) => {
                println!("Nothing received, let's wait a bit...");
                thread::sleep(Duration::from_millis(50));
            },
            // join点 发送端完成所有数据的发送后会被丢弃 此时接收端会收到一个Disconnected错误
            // 也就是说主线程会在该错误发生处等待子线程结束
            Err(TryRecvError::Disconnected) => {
                println!("Sender disconnected");
                break;
            }
        }
    }
}
