#[derive(Debug)]
enum Status {
    Value(u32),
    Stop,
}

fn main() {
    // Status::Value(3) 看起来和函数调用有些相似
    // 而实际上这种构造器确实被实现为了一个函数
    // 该函数接收一个参数并返回一个 Status::Value 变体
    let v = Status::Value(3);

    let list_of_statuses: Vec<Status> = (0u32..20).
        // 所以可以把构造器作为实现了闭包Trait的函数指针来使用
        map(Status::Value).
        collect();

    println!("{:?}", list_of_statuses);
}
