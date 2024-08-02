use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    // 子线程中运行的闭包没有使用move关键字 但使用了主线程中的数据
    let join_handle = thread::spawn(|| {  // error: closure may outlive the current function, but it borrows `v`, which is owned by the current function
        println!("Here's a vector: {:?}", v);
    });

    join_handle.join().unwrap();
}
