# PART1. 消息传递

- 一种很流行且能保证安全并发的计数就是: **消息传递**
- 线程(或Actor)通过彼此发送消息(数据)来进行通信

Actor模型是一种并发计算的数学模型,它将并发计算的实体抽象为一系列的"actors".每个actor都可以执行以下操作:

1. 创建更多的actors
2. 发送消息给其他actors
3. 决定如何响应接收到的下一个消息(actors可以修改自己的私有状态)

在Actor模型中,actors是并发的基本单位,每个actor都有自己的私有状态和行为,并且actors之间仅通过消息传递进行通信,没有共享状态,这样可以避免并发编程中常见的问题,如竞态条件和死锁.

消息传递通常是异步的,actor处理消息的顺序是不确定的

Actor模型的优点包括:

- **高并发**:由于actors之间的解耦和异步消息传递,系统可以轻松地在多核或分布式环境中实现高并发
- **容错性**:actors可以监控其他actors的失败,并采取恢复措施,从而提高系统的容错性
- **可伸缩性**:系统可以通过增加更多的actors来水平扩展,以处理更高的负载

Rust语言中并没有内置的Actor模型支持,但可以通过使用库如`actix`来实现Actor模型

GO语言的名言: **不要用共享内存来通信,要用通信来共享内存**

Rust也提供了一种基于消息传递的并发方式:Channel(标准库提供)

# PART2. Channel

一个Channel包含发送端和接收端:

- 调用发送端的方法可以发送数据
- 接收端会检查和接收到达的数据
- 如果发送端、接收端中的任意一端被丢弃了,那么Channel就"关闭"了

# PART3. 创建Channel

- 使用`mpsc::channel()`函数可以创建一个Channel
  - 该函数返回一个元组,包含发送端和接收端
  - `mpsc`: multiple producer, single consumer(多个生产者,单个消费者)
  - 也就是说,可以有多个发送端,但只能有一个接收端

```rust
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
```

```
cargo run
   Compiling channel_example_1 v0.1.0 (/channel_example_1)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.85s
     Running `target/debug/channel_example_1`
Got: hi
```

# PART4. `Sender<T>.send()`方法

- 参数: 想要发送的数据
- 返回值: `Result<(), SendError<T>>`
  - 如果接收端被丢弃了,则返回`Err(SendError<T>)`
  - 否则返回`Ok(())`

# PART5. 接收端的方法

- `Receiver<T>.recv()`方法:阻止当前线程执行,直到Channel中有数据到来
  - 返回值: `Result<T, RecvError>`
    - 如果发送端被丢弃了,则返回`Err(RecvError)`
    - 否则返回`Ok(T)`
- `Receiver<T>.try_recv()`方法:不会阻止当前线程执行
  - 如果Channel中有数据到来,则返回`Ok(T)`
  - 否则返回`Err(TryRecvError)`
  - 通常会使用循环调用来检查`Receiver<T>.try_recv()`的结果

```rust
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
```

```
cargo run
   Compiling try_recv_example_1 v0.1.0 (/try_recv_example_1)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.22s
     Running `target/debug/try_recv_example_1`
Nothing received, let's wait a bit...
Received: 1
Nothing received, let's wait a bit...
Received: 2
Nothing received, let's wait a bit...
Nothing received, let's wait a bit...
Received: 3
Nothing received, let's wait a bit...
Nothing received, let's wait a bit...
Received: 4
Nothing received, let's wait a bit...
Nothing received, let's wait a bit...
Received: 5
Nothing received, let's wait a bit...
Nothing received, let's wait a bit...
Sender disconnected
```

# PART6. Channel和所有权转移

- 所有权在消息传递中非常重要:它能帮你编写安全、并发的代码

```rust
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
```

```
cargo run
   Compiling ownership_move_after_send_example_1 v0.1.0 (/ownership_move_after_send_example_1)
error[E0382]: borrow of moved value: `val`
  --> src/main.rs:10:32
   |
8  |         let val = String::from("hi");
   |             --- move occurs because `val` has type `String`, which does not implement the `Copy` trait
9  |         tx.send(val).unwrap();  // 此处val的所有权被转移 转移给了接收端
   |                 --- value moved here
10 |         println!("val is: {}", val);  // error: value borrowed here after move
   |                                ^^^ value borrowed here after move
   |
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider cloning the value if the performance cost is acceptable
   |
9  |         tx.send(val.clone()).unwrap();  // 此处val的所有权被转移 转移给了接收端
   |                    ++++++++

For more information about this error, try `rustc --explain E0382`.
error: could not compile `ownership_move_after_send_example_1` (bin "ownership_move_after_send_example_1") due to 1 previous error
```

# PART7. 发送多个值,观察接收端在等待

```rust
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
```

```
cargo run
   Compiling receiver_waiting_example_1 v0.1.0 (/receiver_waiting_example_1)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.61s
     Running `target/debug/receiver_waiting_example_1`
Got: hi
Got: from
Got: the
Got: thread
```

# PART8. 通过克隆创建多个发送者

```rust
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = channel();
    // 克隆一个 Sender 实例 这2个Sender实例共同向同一个Channel中发送消息
    let tx1 = Sender::clone(&tx);

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("1: hi"),
            String::from("1: from"),
            String::from("1: the"),
            String::from("1: thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
```

```
cargo run
   Compiling clone_sender_example_1 v0.1.0 (/clone_sender_example_1)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.00s
     Running `target/debug/clone_sender_example_1`
Got: hi
Got: 1: hi
Got: from
Got: 1: from
Got: the
Got: 1: the
Got: thread
Got: 1: thread
```