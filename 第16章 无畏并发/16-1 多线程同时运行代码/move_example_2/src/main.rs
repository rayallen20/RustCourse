use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let join_handle = thread::spawn(|| {
        println!("Here's a vector: {:?}", v);
    });

    drop(v); // 提前释放v 由于子线程和主线程是并行的 因此有可能出现的情况是: 子线程在主线程释放v之后才开始执行

    join_handle.join().unwrap();
}
