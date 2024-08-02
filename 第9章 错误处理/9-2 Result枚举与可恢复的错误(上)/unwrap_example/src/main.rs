use std::fs::File;

fn main() {
    // 以下2种方式是完全等效的
    match_result();
    unwrap_result();
}

fn match_result() {
    let f = File::open("hello.txt");
    let opening_file = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("{:?}", error)
        }
    };
}

fn unwrap_result() {
    // unwrap()方法是match的一个简写
    // 如果Result是Ok,unwrap()会返回Ok中的值
    // 如果Result是Err, unwrap()会调用panic!宏
    let f = File::open("hello.txt").unwrap();
}
