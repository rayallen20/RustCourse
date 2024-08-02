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
    // 这行代码等价于下面的 match 表达式
    let mut opening_file = File::open("hello.txt")?;

    // let mut opening_file = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    let mut s = String::new();

    // 这行代码等价于下面的 match 表达式
    // 这里我们并没有用到Ok()中的值 所以不需要使用变量接收
    opening_file.read_to_string(&mut s)?;

    // match opening_file.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }

    Ok(s)
}
