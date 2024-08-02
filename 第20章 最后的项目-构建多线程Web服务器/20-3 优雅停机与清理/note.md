# PART1. 为`ThreadPool`实现`Drop` trait

所有的线程都应当在线程池被丢弃时调用`join()`方法,从而确保它们能够在结束前完成自己的工作

`src/pool/worker.rs`:

```rust
use std::sync::{Arc, mpsc, Mutex};
use std::thread;
use crate::pool::Job;

pub(crate) struct Worker {
    id: usize,
    pub(crate) thread: thread::JoinHandle<()>
}

impl Worker {
    pub(crate) fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();
                println!("Worker {} got a job; executing.", id);
                job();
            }
        });
        Worker {
            id,
            thread
        }
    }
}
```

注:此处需把`thread`字段的访问修饰符改为`pub(crate)`,以便`ThreadPool`能够在`Drop` trait中访问它

`src/pool/thread_pool.rs`:

```rust
use std::sync::{Arc, mpsc, Mutex};
use crate::pool::Job;
use crate::pool::Worker;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>
}

impl ThreadPool {
    /// 创建一个新的线程池
    /// size: 线程池中线程的数量
    /// # Panics
    /// 关联`new()`在`size`为0时会触发panic
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, receiver.clone()));
        }

        ThreadPool {
            workers,
            sender
        }
    }

    pub fn execute<F>(&self, f :F)
    where
        F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap()
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            worker.thread.join().unwrap();
        }
    }
}
```

这段代码是无法通过编译的,因为`join()`方法要求取得`JoinHandle`实例的所有权.

```
cargo check
    Checking my_web_server v0.1.0 (/my_web_server)
error[E0507]: cannot move out of `worker.thread` which is behind a mutable reference
    --> src/pool/thread_pool.rs:43:13
     |
43   |             worker.thread.join().unwrap();
     |             ^^^^^^^^^^^^^ ------ `worker.thread` moved due to this method call
     |             |
     |             move occurs because `worker.thread` has type `JoinHandle<()>`, which does not implement the `Copy` trait
     |
note: `JoinHandle::<T>::join` takes ownership of the receiver `self`, which moves `worker.thread`
    --> /.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/src/rust/library/std/src/thread/mod.rs:1657:17
     |
1657 |     pub fn join(self) -> Result<T> {
     |                 ^^^^

For more information about this error, try `rustc --explain E0507`.
error: could not compile `my_web_server` (lib) due to 1 previous error
```

很明显,现在的代码只是可变地借用了`worker`,并没有获取其所有权.为了解决这个问题,我们需要使用`Option<JoinHandle<()>>`来包装`thread`字段,并在`Drop` trait中将其取出(`Option.take()`方法)

`src/pool/worker.rs`:

```rust
use std::sync::{Arc, mpsc, Mutex};
use std::thread;
use crate::pool::Job;

pub(crate) struct Worker {
    pub(crate) id: usize,
    pub(crate) thread: Option<thread::JoinHandle<()>>
}

impl Worker {
    pub(crate) fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();
                println!("Worker {} got a job; executing.", id);
                job();
            }
        });
        Worker {
            id,
            thread: Some(thread)
        }
    }
}
```

注意:这里把`id`字段也改成了包内可访问的,以便`ThreadPool`能够在`Drop` trait中访问它

`src/pool/thread_pool.rs`:

```rust
use std::sync::{Arc, mpsc, Mutex};
use crate::pool::Job;
use crate::pool::Worker;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>
}

impl ThreadPool {
    /// 创建一个新的线程池
    /// size: 线程池中线程的数量
    /// # Panics
    /// 关联`new()`在`size`为0时会触发panic
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, receiver.clone()));
        }

        ThreadPool {
            workers,
            sender
        }
    }

    pub fn execute<F>(&self, f :F)
    where
        F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap()
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
```

```
cargo check
    Checking my_web_server v0.1.0 (/my_web_server)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.19s
```

# PART2. 通知线程停止监听任务

实际上现在我们并没有实现停机的功能.因为虽然主线程在等待`join()`,但实际上子线程都还在`loop`的死循环中,无法跳出这个循环.

因此我们要做的是:定义一个信号,在线程池被丢弃时发送这个信号,以通知所有的工作线程停止监听任务;而子线程接收到这个信号则跳出循环

## 2.1 定义信号

`src/pool/message.rs`:

```rust
use crate::pool::Job;

pub(crate) enum Message {
    NewJob(Job),
    Terminate
}
```

`src/pool/mod.rs`:

```rust
pub mod thread_pool;
pub use thread_pool::ThreadPool;

mod worker;
use worker::Worker;

mod job;
use job::Job;

mod message;
use message::Message;
```

## 2.2 修改channel的类型

### 2.2.1 修改`ThreadPool`的`sender`字段和`execute()`方法

`src/pool/thread_pool.rs`:

- step1. 修改`sender`字段的类型
- step2. 在`execute()`方法中发送`Message::NewJob`消息

```rust
use std::sync::{Arc, mpsc, Mutex};
use crate::pool::Message;
use crate::pool::Worker;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>
}

impl ThreadPool {
    /// 创建一个新的线程池
    /// size: 线程池中线程的数量
    /// # Panics
    /// 关联`new()`在`size`为0时会触发panic
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, receiver.clone()));
        }

        ThreadPool {
            workers,
            sender
        }
    }

    pub fn execute<F>(&self, f :F)
    where
        F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap()
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
```

### 2.2.2 修改`Worker`的`receiver`字段和`loop`循环

`src/pool/worker.rs`:

- step1. 修改`receiver`字段的类型
- step2. 在`loop`循环中对`Message`枚举的变体进行匹配

```rust
use std::sync::{Arc, mpsc, Mutex};
use std::thread;
use crate::pool::Message;

pub(crate) struct Worker {
    pub(crate) id: usize,
    pub(crate) thread: Option<thread::JoinHandle<()>>
}

impl Worker {
    pub(crate) fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();
                match message { 
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.", id);
                        job();
                    },
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);
                        break;
                    }
                }
            }
        });
        Worker {
            id,
            thread: Some(thread)
        }
    }
}
```

### 2.2.3 修改`ThreadPool`的`Drop` trait

```rust
use std::sync::{Arc, mpsc, Mutex};
use crate::pool::Message;
use crate::pool::Worker;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>
}

impl ThreadPool {
    /// 创建一个新的线程池
    /// size: 线程池中线程的数量
    /// # Panics
    /// 关联`new()`在`size`为0时会触发panic
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, receiver.clone()));
        }

        ThreadPool {
            workers,
            sender
        }
    }

    pub fn execute<F>(&self, f :F)
    where
        F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap()
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // 发送终止信号给每个线程
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap()
        }
        
        // 等待每个线程终止
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
        
        // 注意: 这里不能1次在遍历中既发送终止信号又等待线程终止
        // 因为主线程是不确定发出的这个终止信号究竟被哪个子线程接收到了
        // 这里我们假设有2个worker(worker1和worker2),主线程在遍历时,先遍历到worker1,
        // 发送了1个终止信号.但此时可能worker1正在执行任务(正在match表达式的NewJob(job)这个分支中),
        // 没有接收到终止信号,而此时恰巧worker2处于空闲,接收到了终止信号
        // 而此时主线程的join()操作中,等待的是worker1的线程,而worker1的线程因为没有收到终止信号,
        // 即使它完成了工作,也不会终止.
        // 因此主线程永远无法发送第2个终止信号,也就无法终止worker1的线程.这就造成了死锁!
        // for worker in &mut self.workers {
        //     // 发送终止信号
        //     self.sender.send(Message::Terminate).unwrap();
        //     
        //     // 等待线程终止
        //     println!("Shutting down worker {}", worker.id);
        //     if let Some(thread) = worker.thread.take() {
        //         thread.join().unwrap();
        //     }
        // }
    }
}
```

```
cargo check
    Checking my_web_server v0.1.0 (/my_web_server)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 4.13s
```

# PART3. 修改`main.rs`,使得服务器在处理2个请求后,丢弃线程池

`src/main.rs`:

```rust
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;
use my_web_server::pool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = pool::ThreadPool::new(4);
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        pool.execute(||{
            handle_connection(stream);
        })
    }
    
    println!("Shutting down.");
}

/// 本函数用于从TCP连接中读取数据并打印请求相关信息
/// 通常来讲,读取操作是不需要mut的.但是TcpStream.read()方法需要一个可变引用(这里的可变引用指的是mut stream: TcpStream)
/// 这是因为TcpStream内部维护了一个缓冲区,每次读取数据时都会将数据写入到这个缓冲区中,因此需要一个可变引用来修改这个缓冲区
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = "GET / HTTP/1.1\r\n";
    let sleep = "GET /sleep HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get.as_bytes()) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep.as_bytes()) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    }
    else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
```

# PART4. 运行

```
cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/my_web_server`
```

请求2次后,控制台输出如下:

```
Worker 0 got a job; executing.
Shutting down.
Shutting down worker 0
Worker 1 got a job; executing.
Worker 2 was told to terminate.
Worker 3 was told to terminate.
Worker 0 was told to terminate.
Shutting down worker 1
Worker 1 was told to terminate.
```