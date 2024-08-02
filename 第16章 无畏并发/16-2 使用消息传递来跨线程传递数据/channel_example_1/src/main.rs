use std::sync::mpsc::channel;
use std::thread;

fn main() {
    // mpsc::channel()关联函数创建的是一个无界通道(无界通道指的是发送端和接收端之间的数据传输是无限制的)
    // 或者可以这么理解:
    // 无界通道就是GO中创建一个不限制长度的channel
    // 有界通道就是GO中创建一个限制长度的channel
    // 但无论使用无界通道还是有界通道,发送端和接收端的行为都是同步的
    // 对于Sender.send()方法而言,它会阻塞当前线程直到通道中有空间可用(也就是阻塞当前线程直到把数据发送出去)
    // 对于Receiver.recv()方法而言,它会阻塞当前线程直到接收到数据(也就是阻塞当前线程直到有数据到来)
    // 而所谓的异步通信,是指发送和接收操作不会阻塞当前线程.而是在如果操作不能立即完成(例如发送端发送数据时通道已满,接收端接收数据时通道为空),
    // 则任务会被挂起,等到操作可以立即完成时再恢复任务
    let (tx, rx) = channel();

    thread::spawn(move || {
        let val = String::from("hi");
        // 线程必须拥有发送端的所有权 才能往通道中发送数据
        // 如果接收端被丢弃了 则Sender.send()方法会返回Result的Err变体
        tx.send(val).unwrap();
    });

    // Receiver.recv()方法会阻塞当前线程,直到有数据到来
    // 换言之 Receiver.recv()方法就是join点
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
