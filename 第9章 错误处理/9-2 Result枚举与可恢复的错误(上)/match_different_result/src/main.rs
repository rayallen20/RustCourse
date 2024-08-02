use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let opening_file = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            // 如果文件不存在,则创建文件
            ErrorKind::NotFound => match File::create("hello.txt") {
                // 如果文件创建成功,则返回文件句柄
                Ok(file) => file,

                // 如果文件创建失败,则 panic
                Err(error) => panic!("Problem creating the file: {:?}", error),
            },

            // 如果文件打开失败,则 panic
            // 这里的 other_error 的作用相当于是一个通配符,匹配所有没有被穷举的情况
            other_error => panic!("Problem opening the file: {:?}", other_error),
        }
    };
}
