# PART1. 现有问题

目前,我们的服务器会依次处理各个请求,这也就意味着**它在处理完第一个请求前不会处理第二个连接**.

服务器接收到的请求越多,这类**串行**操作就会使整体性能越差.

当服务器接收到某个需要处理很长时间的请求时,其余的请求就不得不排队进行等待,即便新请求可以被快速处理完毕.

# PART2. 模拟一个慢请求

需求:当请求`/sleep`时,服务器会等待5秒后再返回响应.

```rust
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
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

此时,在访问`http://127.0.0.1:7878/sleep` 时,再访问其他uri,则会一直等待,直到`/sleep`请求处理完才会被处理.

# PART3. 使用线程池改进吞吐量

## 3.1 为每个流创建了独立的新线程

```rust
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        
        // 为每个连接创建一个线程来处理请求
        thread::spawn(||{
            handle_connection(stream);
        });
    }
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

但这样的缺点在于:每个请求都会创建一个新线程,这会导致线程数的快速增长,最终会耗尽系统资源.

## 3.2 使用线程池改进

这里我们采用**编译器驱动开发**的模式,先假定我们已经实现了一个线程池,然后再去实现它.

这样做的好处在于:**避免在切换方案时对使用该API的代码做出较大的修改**

## 3.3 定义线程池结构体

```
tree ./
./
├── 404.html
├── Cargo.lock
├── Cargo.toml
├── hello.html
└── src
    ├── lib.rs
    ├── main.rs
    └── pool
        ├── mod.rs
        └── thread_pool.rs

2 directories, 8 files
```

`src/pool/thread_pool.rs`

```rust
pub struct ThreadPool;
```

`src/pool/mod.rs`:

```rust
pub mod thread_pool;
pub use thread_pool::ThreadPool;
```

`src/lib.rs`:

```rust
pub mod pool;
```

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
    // TODO: 本关联函数尚未实现
    let pool = pool::ThreadPool::new(4);
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // TODO: 本方法尚未实现
        pool.execute(||{
            handle_connection(stream);
        })
    }
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

## 3.4 定义线程池的创建

```
cargo check
    Checking my_web_server v0.1.0 (/my_web_server)
error[E0599]: no function or associated item named `new` found for struct `ThreadPool` in the current scope
  --> src/main.rs:11:34
   |
11 |     let pool = pool::ThreadPool::new(4);
   |                                  ^^^ function or associated item not found in `ThreadPool`

For more information about this error, try `rustc --explain E0599`.
error: could not compile `my_web_server` (bin "my_web_server") due to 1 previous error
```

已经找到了第1个问题:没有实现关联函数`ThreadPool::new()`

`src/pool/thread_pool.rs`:

```rust
pub struct ThreadPool;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        ThreadPool
    }
}
```

## 3.5 定义`execute()`方法

继续检查:

```
cargo check
    Checking my_web_server v0.1.0 (/my_web_server)
warning: unused variable: `size`
 --> src/pool/thread_pool.rs:4:16
  |
4 |     pub fn new(size: usize) -> ThreadPool {
  |                ^^^^ help: if this is intentional, prefix it with an underscore: `_size`
  |
  = note: `#[warn(unused_variables)]` on by default

warning: `my_web_server` (lib) generated 1 warning
error[E0599]: no method named `execute` found for struct `ThreadPool` in the current scope
  --> src/main.rs:16:14
   |
16 |         pool.execute(||{
   |         -----^^^^^^^ method not found in `ThreadPool`

For more information about this error, try `rustc --explain E0599`.
error: could not compile `my_web_server` (bin "my_web_server") due to 1 previous error
```

此处我们先不管warning,先看error.很明显这里我们没有实现`execute()`方法.

`execute()`方法的签名,理论上来讲,应该和`thread::spawn()`关联函数是类似的:

```rust
#[stable(feature = "rust1", since = "1.0.0")]
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
where
    F: FnOnce() -> T,
    F: Send + 'static,
    T: Send + 'static,
{
    Builder::new().spawn(f).expect("failed to spawn thread")
}
```

这里我们先不用管泛型参数`T`,因为它只和返回值有关.这里要关注的是泛型参数`F`,它的类型约束为`FnOnce() -> T + Send`,要求其生命周期为`'static`

这里多提一嘴:之所以`F`的生命周期被要求为`'static`,原因如下:

1. 线程独立性:新创建的线程是独立于创建它的线程运行的.主线程可能会在子线程之前退出,这样就会导致被捕获的引用失效 
2. 避免悬挂引用:如果允许非`'static`生命周期的引用传递给新线程,那么当这些引用所指向的内存被释放后,子线程仍然可能会尝试访问这些内存,从而导致未定义行为(比如悬挂指针)
3. 内存安全:Rust的所有权和生命周期系统的一个核心目标是确保内存安全.要求`'static`生命周期可以确保子线程不会持有任何悬挂引用,从而防止一类常见的并发编程错误

在我们的需求中,`exectue()`方法执行的闭包,并不需要返回什么内容,所以我们可以将`F`的泛型约束定义为`FnOnce + Send + 'static`:

```rust
pub struct ThreadPool;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        ThreadPool
    }

    pub fn execute<F>(&self, f :F)
    where
        F: FnOnce() + Send + 'static
    {
    }
}
```

```
cargo check
    Checking my_web_server v0.1.0 (/my_web_server)
warning: unused variable: `size`
 --> src/pool/thread_pool.rs:4:16
  |
4 |     pub fn new(size: usize) -> ThreadPool {
  |                ^^^^ help: if this is intentional, prefix it with an underscore: `_size`
  |
  = note: `#[warn(unused_variables)]` on by default

warning: unused variable: `f`
 --> src/pool/thread_pool.rs:8:30
  |
8 |     pub fn execute<F>(&self, f :F)
  |                              ^ help: if this is intentional, prefix it with an underscore: `_f`

warning: `my_web_server` (lib) generated 2 warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.15s
```

此时,我们的代码已经能够顺利通过编译了.

# PART4. 实现

## 4.1 实现`ThreadPool::new()`

### 4.1.1 判断`size`的合法性,并为该关联函数增加文档

```rust
pub struct ThreadPool;

impl ThreadPool {
    /// 创建一个新的线程池
    /// size: 线程池中线程的数量
    /// # Panics
    /// 关联`new()`在`size`为0时会触发panic
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        ThreadPool
    }

    pub fn execute<F>(&self, f :F)
    where
        F: FnOnce() + Send + 'static
    {
    }
}
```

### 4.1.2 实现`execute()`方法

还是看`thread::spawn()`关联函数的实现:

```rust
#[stable(feature = "rust1", since = "1.0.0")]
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
where
    F: FnOnce() -> T,
    F: Send + 'static,
    T: Send + 'static,
{
    Builder::new().spawn(f).expect("failed to spawn thread")
}
```

该函数返回`JoinHandle<T>`.该类型用于等待线程的完成并检索线程的返回值.因此,我们的线程池也可以使用这个类型:

```rust
use std::thread::JoinHandle;

pub struct ThreadPool {
    threads: Vec<JoinHandle<()>>
}

impl ThreadPool {
    /// 创建一个新的线程池
    /// size: 线程池中线程的数量
    /// # Panics
    /// 关联`new()`在`size`为0时会触发panic
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let mut threads = Vec::with_capacity(size);
        
        for _ in 0..size {
            // 创建线程并将其存储在threads中
        }
        
        ThreadPool {
            threads
        }
    }

    pub fn execute<F>(&self, f :F)
    where
        F: FnOnce() + Send + 'static
    {
    }
}
```

# PART5. 定义Worker结构体

`thread::spawn()`它会在线程创建完毕后立即执行自己接收到的闭包

然而我们需要的是:线程在创建后进入等待状态并执行随后传递给它的闭包

我们会在ThreadPool与线程之间引入一个新的数据结构`Worker`来实现并管理线程的等待和执行

每个`Worker`实例维护自己的`JoinHandle<()>`实例.另外,为了便于调试,我们给每个`Worker`实例分配一个id

`src/pool/worker.rs`:

```rust
use std::thread;

pub(crate) struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>
}

impl Worker {
    pub(crate) fn new(id: usize) -> Worker {
        let thread = thread::spawn(||{});
        Worker {
            id,
            thread
        }
    }
}
```

注:

- 实际上这里创建的线程,由于传入的是一个空闭包,所以这个线程实际上仍旧会被创建,但创建后会立即退出
- 被`pub(crate)`修饰的结构体,只能被同一个crate中的其他模块访问,不能被外部crate访问
  - 本例中,`Worker`和`Worker::new()`,就只能被`ThreadPool`访问,而不能被`main.rs`访问

`src/pool/mod.rs`:

```rust
pub mod thread_pool;
pub use thread_pool::ThreadPool;

mod worker;
use worker::Worker;
```

注:这里我们期望的是`Worker`类型只能在包内被访问,因此不能使用`pub use`将其导出.此处的导入是为了在`ThreadPool`中使用`Worker`(其实不导入也能用,就是`use`会很长)

`src/pool/thread_pool.rs`:

```rust
use crate::pool::Worker;

pub struct ThreadPool {
    workers: Vec<Worker>
}

impl ThreadPool {
    /// 创建一个新的线程池
    /// size: 线程池中线程的数量
    /// # Panics
    /// 关联`new()`在`size`为0时会触发panic
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id))
        }

        ThreadPool {
            workers
        }
    }

    pub fn execute<F>(&self, f :F)
    where
        F: FnOnce() + Send + 'static
    {
    }
}
```

# PART6. 实现`execute()`方法

这里我们预期的是:`Worker`结构体能够从`ThreadPool`中获取闭包,并将这个闭包发送到它的线程中执行

channel这个数据类型,就可以用于线程间通信

这里我们要实现的步骤具体为:

1. `ThreadPool`创建channel,并持有channel的发送端
2. 每个`Worker`都持有channel的接收端
3. 创建一个新的`Job`结构体来持有需要发送到channel中的闭包
4. 在`execute()`方法中,把闭包封装到`Job`结构体中,并发送到channel中
5. `Worker`在自己的线程中不断查询channel,并执行接收到的闭包

## 6.1 定义`Job`结构体

`src/pool/job.rs`:

```rust
pub(crate) struct Job;
```

`src/pool/mod.rs`:

```rust
pub mod thread_pool;
pub use thread_pool::ThreadPool;

mod worker;
use worker::Worker;

mod job;
use job::Job;
```

## 6.2 `ThreadPool`创建channel,并持有channel的发送端

```rust
use std::sync::mpsc;
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
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id))
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
    }
}
```

## 6.3 每个`Worker`都持有channel的接收端

这一步是不能直接把`receiver`传递给每一个`Worker`实例的.原因有以下几点:

1. 所有权问题:把`receiver`给了第1个`Worker`,就没法给第2个`Worker`了
2. `mpsc::Receiver`这个类型没有实现`Copy` Trait,所以不能直接复制;就算它实现了,也不应该复制,因为我们想要的是多个`Worker`持有同一个`mpsc::Receiver`
3. 由于`Worker`需要从`mpsc::Receiver`取出`Job`,因此这要求`mpsc::Receiver`是可变的,但是`mpsc::Receiver`并不是线程安全的

也就是说,如果我们按如下方式修改代码,是有问题的:

`src/pool/worker.rs`:

```rust
use std::sync::mpsc;
use std::thread;
use crate::pool::Job;

pub(crate) struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>
}

impl Worker {
    pub(crate) fn new(id: usize, receiver: mpsc::Receiver<Job>) -> Worker {
        let thread = thread::spawn(||{
            receiver;
        });
        Worker {
            id,
            thread
        }
    }
}
```

`src/pool/thread_pool.rs`:

```rust
use std::sync::mpsc;
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
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, receiver));
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
    }
}
```

```
cargo check
    Checking my_web_server v0.1.0 (/my_web_server)
error[E0382]: use of moved value: `receiver`
  --> src/pool/thread_pool.rs:21:42
   |
17 |         let (sender, receiver) = mpsc::channel();
   |                      -------- move occurs because `receiver` has type `std::sync::mpsc::Receiver<Job>`, which does not implement the `Copy` trait
...
20 |         for id in 0..size {
   |         ----------------- inside of this loop
21 |             workers.push(Worker::new(id, receiver));
   |                                          ^^^^^^^^ value moved here, in previous iteration of loop
   |
...

For more information about this error, try `rustc --explain E0382`.
warning: `my_web_server` (lib) generated 1 warning
error: could not compile `my_web_server` (lib) due to 1 previous error; 1 warning emitted
```

解决方法:

- `Arc<T>`: 允许多个线程访问同一个`T`实例
- `Mutex<T>`: 保证了在任意时刻,只有一个线程能够访问`T`实例

`src/pool/worker.rs`:

```rust
use std::sync::{Arc, mpsc, Mutex};
use std::thread;
use crate::pool::Job;

pub(crate) struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>
}

impl Worker {
    pub(crate) fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(||{
            receiver;
        });
        Worker {
            id,
            thread
        }
    }
}
```

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
    }
}
```

## 6.4 在`execute()`方法中,把闭包封装到`Job`结构体中,并发送到channel中

这里之所以必须定义`Job`类型,是因为我们无法直接将一个闭包发送到channel中,因为闭包的大小是未知的.

这里我们假设把代码改成如下实现:

`src/pool/thread_pool.rs`:

```rust
use std::sync::{Arc, mpsc, Mutex};
use crate::pool::Worker;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<dyn FnOnce() + Send + 'static>
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
            // workers.push(Worker::new(id, receiver.clone()));
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
        self.sender.send(f).unwrap()
    }
}
```

```
cargo check
    Checking my_web_server v0.1.0 (/my_web_server)
error[E0277]: the size for values of type `(dyn FnOnce() + Send + 'static)` cannot be known at compilation time
   --> src/pool/thread_pool.rs:6:13
    |
6   |     sender: mpsc::Sender<dyn FnOnce() + Send + 'static>
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
    |
    = help: the trait `Sized` is not implemented for `(dyn FnOnce() + Send + 'static)`
note: required by an implicit `Sized` bound in `Sender`
...
Some errors have detailed explanations: E0277, E0599.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `my_web_server` (lib) due to 3 previous errors
```

这是因为channel要求发送的类型是大小已知的.

所以很明显,我们定义的`Job`类型,其实应该就是`Box<dyn FnOnce() + Send + 'static>`类型的别名

因为`Box<T>`类型的大小是已知的

`src/pool/job.rs`:

```rust
pub(crate) type Job = Box<dyn FnOnce() + Send + 'static>;
```

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
```

## 6.5 `Worker`在自己的线程中不断查询channel,并执行接收到的闭包

`src/pool/worker.rs`:

```rust
use std::sync::{Arc, mpsc, Mutex};
use std::thread;
use crate::pool::Job;

pub(crate) struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>
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

# PART7. 运行

```
cargo run
   Compiling my_web_server v0.1.0 (/my_web_server)
warning: field `workers` is never read
 --> src/pool/thread_pool.rs:6:5
  |
5 | pub struct ThreadPool {
  |            ---------- field in this struct
6 |     workers: Vec<Worker>,
  |     ^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: fields `id` and `thread` are never read
 --> src/pool/worker.rs:6:5
  |
5 | pub(crate) struct Worker {
  |                   ------ fields in this struct
6 |     id: usize,
  |     ^^
7 |     thread: thread::JoinHandle<()>
  |     ^^^^^^

    Building [                             ] 0/2: my_web_server                                                                                                                                                                      
warning: `my_web_server` (lib) generated 2 warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 6.21s
     Running `target/debug/my_web_server`
```

此时浏览器请求后,控制台输出如下:

```
Worker 3 got a job; executing.
Worker 0 got a job; executing.
Worker 2 got a job; executing.
Worker 1 got a job; executing.
Worker 3 got a job; executing.
```

且访问`http://127.0.0.1:7878/sleep` 不影响访问`http://127.0.0.1:7878/` (只要不连续4次访问`http://127.0.0.1:7878/sleep`)

# PART8. 注意事项

在`src/pool/worker.rs`中,如果你按照如下写法:

```rust
use std::sync::{Arc, mpsc, Mutex};
use std::thread;
use crate::pool::Job;

pub(crate) struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>
}

impl Worker {
    pub(crate) fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            // loop {
            //     let job = receiver.lock().unwrap().recv().unwrap();
            //     println!("Worker {} got a job; executing.", id);
            //     job();
            // }

            while let Ok(job) = receiver.lock().unwrap().recv() {
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

那么你请求`http://127.0.0.1:7878/sleep` 是会影响你请求`http://127.0.0.1:7878/` 的.也就是又变为了串行处理请求

这是因为:

1. `Mutex<T>`没有公开的`unlock()`方法,只能在`Mutex.lock()`方法返回的`MutexGuard`离开作用域时自动释放锁
2. 使用表达式`let job = receiver.lock().unwrap().recv().unwrap();`,该表达式的作用域仅此一行.因此`receiver.lock()`方法返回的`MutexGuard<'_, T>`在这一行结束时,锁就已经释放了
   - 换言之,后续的`job()`调用,并不是在锁的保护下执行的,因此不会影响锁的释放
3. 使用表达式`while let Ok(job) = receiver.lock().unwrap().recv()`,则整个`while let`代码块都是该表达式的作用域.
   - 也就是说,后续的`job()`调用,是在锁的保护下执行的,会影响锁的释放