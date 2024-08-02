use std::sync::mpsc::channel;
use std::thread;

fn main() {
    let (tx, rx) = channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();  // 此处val的所有权被转移 转移给了接收端
        println!("val is: {}", val);  // error: value borrowed here after move
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
