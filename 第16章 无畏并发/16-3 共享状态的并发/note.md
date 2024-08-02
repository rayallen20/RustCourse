# PART1. 使用共享来实现并发

GO语言名言: **不要用共享内存来通信,要用通信来共享内存**

上一节中,就是**用通信来共享内存**的

Rust支持通过共享状态来实现并发

- Channel类似单所有权: 一旦将值的所有权转移给了通道,就不能再使用这个值了
- 共享内存并发类似多所有权: 多个线程可以同时访问同一块内存

# PART2. 使用`Mutex`来每次只允许1个线程来访问数据

- `Mutex`:是mutual exclusion(互斥)的缩写
- 在同一时刻,`Mutex`只允许1个线程来访问某些数据
- 想要访问数据:
  - 线程必须首先获取互斥锁(lock)
  - lock数据结构是`Mutex`的一部分,它能跟踪谁对数据拥有独占访问权(也就是此刻哪个线程持有锁)
- `Mutex`通常被描述为:通过锁定系统来保护它所持有的数据

# PART3. `Mutex`的2条规则

- 使用数据之前,必须尝试获取锁(lock)
- 使用完`Mutex`所保护的数据,必须对数据进行解锁,以便其他线程可以获取锁

# PART4. `Mutex<T>`的API

- `Mutex::new(t: T) -> Mutex<T>`: 创建一个新的`Mutex<T>`
  - 其中`t`就是`Mutex`要保护的数据
  - `Mutex<T>`是一个智能指针
- 访问数据前,使用`lock()`方法获取锁
  - 该方法会阻塞当前线程
  - 该方法可能会失败
  - 该方法返回一个`MutexGuard<T>`的智能指针,因此我们可以访问其内部的数据

```rust
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        // m.lock()方法的返回值类型为Result<MutexGuard<T>, PoisonError<MutexGuard<T>>>
        // MutexGuard<T>是一个智能指针，实现了Deref和Drop trait,因此可以获得其内部数据
        let mut num = m.lock().unwrap();
        *num = 6;
    } // MutexGuard<T>是智能指针,因此离开作用域时会自动Drop,该类型的drop()方法会自动释放锁

    println!("m = {:?}", m);
}
```

```
cargo run
   Compiling mutex_example_1 v0.1.0 (/mutex_example_1)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.89s
     Running `target/debug/mutex_example_1`
m = Mutex { data: 6, poisoned: false, .. }
```

# PART5. 多线程共享`Mutex<T>`

```rust
use std::sync::Mutex;
use std::thread;
use std::thread::JoinHandle;

fn main() {
    let counter = Mutex::new(0);
    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    // fork点
    for _ in 1..10 {
        // 这里的错误在于 当第1个线程获取了锁的所有权之后 后续的线程就无法再获取锁的所有权了
        let handle = thread::spawn(move || { // error: value moved after being moved
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
```

```
cargo run
   Compiling share_mutex_example_1 v0.1.0 (/share_mutex_example_1)
error[E0382]: borrow of moved value: `counter`
  --> src/main.rs:24:29
   |
6  |     let counter = Mutex::new(0);
   |         ------- move occurs because `counter` has type `Mutex<i32>`, which does not implement the `Copy` trait
...
12 |         let handle = thread::spawn(move || { // error: value moved after being moved
   |                                    ------- value moved into closure here, in previous iteration of loop
...
24 |     println!("Result: {}", *counter.lock().unwrap());
   |                             ^^^^^^^ value borrowed here after move

For more information about this error, try `rustc --explain E0382`.
error: could not compile `share_mutex_example_1` (bin "share_mutex_example_1") due to 1 previous error
```

为了让错误更清晰一些,我们改一下代码:

```rust
use std::sync::Mutex;
use std::thread;
use std::thread::JoinHandle;

fn main() {
  let counter = Mutex::new(0);
  let mut handles: Vec<JoinHandle<()>> = Vec::new();

  // fork点1
  let handle1 = thread::spawn(move || {
    let mut num = counter.lock().unwrap();
    *num += 1;
  });
  handles.push(handle1);

  // fork点2
  let handle2 = thread::spawn(move || { // error: value moved into closure here, in previous iteration of loop
    let mut num = counter.lock().unwrap();
    *num += 1;
  });
  handles.push(handle2);

  // join点
  for handle in handles {
    handle.join().unwrap();
  }

  println!("Result: {}", *counter.lock().unwrap()); // error: value borrowed here after move
}
```

```
cargo run
   Compiling share_mutex_example_2 v0.1.0 (/share_mutex_example_2)
error[E0382]: use of moved value: `counter`
  --> src/main.rs:17:33
   |
6  |     let counter = Mutex::new(0);
   |         ------- move occurs because `counter` has type `Mutex<i32>`, which does not implement the `Copy` trait
...
10 |     let handle1 = thread::spawn(move || {
   |                                 ------- value moved into closure here
11 |         let mut num = counter.lock().unwrap();
   |                       ------- variable moved due to use in closure
...
17 |     let handle2 = thread::spawn(move || { // error: value moved into closure here, in previous iteration of loop
   |                                 ^^^^^^^ value used here after move
18 |         let mut num = counter.lock().unwrap();
   |                       ------- use occurs due to use in closure

error[E0382]: borrow of moved value: `counter`
  --> src/main.rs:28:29
   |
6  |     let counter = Mutex::new(0);
   |         ------- move occurs because `counter` has type `Mutex<i32>`, which does not implement the `Copy` trait
...
17 |     let handle2 = thread::spawn(move || { // error: value moved into closure here, in previous iteration of loop
   |                                 ------- value moved into closure here
18 |         let mut num = counter.lock().unwrap();
   |                       ------- variable moved due to use in closure
...
28 |     println!("Result: {}", *counter.lock().unwrap());
   |                             ^^^^^^^ value borrowed here after move

For more information about this error, try `rustc --explain E0382`.
error: could not compile `share_mutex_example_2` (bin "share_mutex_example_2") due to 2 previous errors
```

# PART6. 多线程的多重所有权

之前讲过一个多重所有权的智能指针`Rc<T>`,但是,`Rc<T>`只能用于单线程,因为它不是线程安全的.

只有实现了`Send` Trait的类型,才能跨线程传递所有权.也就是说`Send` Trait确保了线程安全

```rust
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
```

```
cargo run                  
   Compiling rc_share_mutex_example_1 v0.1.0 (/rc_share_mutex_example_1)
error[E0277]: `Rc<Mutex<i32>>` cannot be sent between threads safely
   --> src/main.rs:13:36
    |
13  |           let handle = thread::spawn(move || {
    |                        ------------- ^------
    |                        |             |
    |  ______________________|_____________within this `{closure@src/main.rs:13:36: 13:43}`
    | |                      |
    | |                      required by a bound introduced by this call
14  | |             let mut num = counter.lock().unwrap();
15  | |             *num += 1;
16  | |         });
    | |_________^ `Rc<Mutex<i32>>` cannot be sent between threads safely
    |
    = help: within `{closure@src/main.rs:13:36: 13:43}`, the trait `Send` is not implemented for `Rc<Mutex<i32>>`, which is required by `{closure@src/main.rs:13:36: 13:43}: Send`
note: required because it's used within this closure
   --> src/main.rs:13:36
    |
13  |         let handle = thread::spawn(move || {
    |                                    ^^^^^^^
note: required by a bound in `spawn`
   --> /.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/src/rust/library/std/src/thread/mod.rs:680:8
    |
677 | pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    |        ----- required by a bound in this function
...
680 |     F: Send + 'static,
    |        ^^^^ required by this bound in `spawn`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `rc_share_mutex_example_1` (bin "rc_share_mutex_example_1") due to 1 previous error
```

重点在于帮助信息中:

help: within `{closure@src/main.rs:13:36: 13:43}`, the trait `Send` is not implemented for `Rc<Mutex<i32>>`, which is required by `{closure@src/main.rs:13:36: 13:43}: Send`
note: required because it's used within this closure

很明显的提示,要求实现了`Send` Trait的类型,才能跨线程传递所有权

# PART7. 使用`Arc<T>`进行原子引用计数

- `Arc<T>`是`Rc<T>`的线程安全版本
- `Arc<T>`是`Atomic Reference Counting`的缩写
- 二者API相同

为什么所有的基础类型都不是原子的? 为什么标准库类型不默认使用`Arc<T>`?

- 原子操作是一种特殊的操作,它们可以在多线程环境下安全地执行
- 原子操作是CPU级别的操作,它们是硬件级别的操作,因此会比普通操作慢

```rust
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
```

```
cargo run                   
   Compiling arc_share_mutex_example_1 v0.1.0 (/arc_share_mutex_example_1)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.59s
     Running `target/debug/arc_share_mutex_example_1`
Result: 9
```

# PART8. 比较`RefCell<T>`&`Rc<T>`和`Mutex<T>`&`Arc<T>`

- `Mutex<T>`提供了内部可变性,这一点和`Cell`家族是一样的
- 可以使用`RefCell<T>`来改变`Rc<T>`的内部值
  - 例如:`let v = Rc<RefCell<5>>`,可以使用 `*v.borrow_mut() += 1`来改变`Rc<T>`的内部值
  - 但是要注意:使用`Rc<T>`可能会产生循环引用,从而导致内存泄漏
- 可以使用`Mutex<T>`来改变`Arc<T>`的内部值
  - 例如:`let v = Arc<Mutex<5>>`,可以使用 `Arc::clone(&v).lock().unwrap() += 1`来改变`Arc<T>`的内部值
  - 但是要注意:`Mutex<T>`有死锁的风险