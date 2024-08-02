use std::fs::File;

fn main() {
    // 以下2种方式是等效的
    match_result();
    expect_result();
}

fn match_result() {
    let f = File::open("hello.txt");
    let opening_file = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("自定义的打开文件错误信息: {:?}", error)
        }
    };
}

fn expect_result() {
    // expect()方法可以自定义错误信息
    let f = File::open("hello.txt").expect("自定义的打开文件错误信息: ");
}