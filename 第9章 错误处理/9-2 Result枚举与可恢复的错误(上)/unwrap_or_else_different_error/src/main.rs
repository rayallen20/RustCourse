use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // unwrap_or_else()方法接收一个闭包作为参数
    // 若Result中的值是Ok, 则返回Ok中的值
    // 若Result中的值是Err, 则调用闭包
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("There was a problem creating the file: {:?}", error);
            })
        } else {
            panic!("There was a problem opening the file: {:?}", error)
        }
    });
}
