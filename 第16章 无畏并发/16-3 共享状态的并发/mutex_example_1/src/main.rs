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
