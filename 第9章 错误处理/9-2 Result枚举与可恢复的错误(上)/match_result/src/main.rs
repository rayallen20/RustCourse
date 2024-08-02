use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let opening_file = match f {
        // 成功打开文件 则返回文件句柄
        Ok(file) => file,

        // 打开文件失败 则打印错误信息
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        },
    };
}
