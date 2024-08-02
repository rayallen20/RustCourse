use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    let username = read_username_from_file();
    match username {
        Ok(s) => println!("The username is: {}", s),
        Err(e) => println!("Error: {}", e),
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut opening_file = match f {
        Ok(file) => file,
        // File::open()函数返回的io::Result中,其Err变体的类型为io::Error
        // 和本函数返回的Result的Err变体类型一致
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    // read_to_string()方法要求File实例的可变引用 因此需要让 opening_file 变量是可变的
    match opening_file.read_to_string(&mut s) {
        // read_to_string()方法返回的io::Result中,其Ok变体的类型为usize 表示读取的字节数
        Ok(_) => Ok(s),

        // read_to_string()方法返回的io::Result中,其Err变体的类型为io::Error
        // 和本函数返回的Result的Err变体类型一致
        Err(e) => Err(e),
    }
    // 最终返回的是这个match表达式的结果 要么是Ok(s) 要么是Err(e)
}
