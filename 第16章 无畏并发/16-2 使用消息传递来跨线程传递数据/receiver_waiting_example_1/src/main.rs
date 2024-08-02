use std::sync::mpsc::channel;
use std::thread;

fn main() {
    let (tx, rx) = channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        // 发送端每隔1s发送一个数据
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    // 此处将接收端作为迭代器使用 不需要每次都调用recv()方法
    // 当Channel被关闭(也就是发送端被drop)时,迭代器会自动结束
    for received in rx {
        println!("Got: {}", received);
    }
}
