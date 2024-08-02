use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    // 创建一个TcpListener并绑定到本地地址
    // TcpListener::bind()关联函数会返回一个Result<T, E>类型的枚举 此处我们简单使用unwrap()方法来处理错误即可
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // listener.incoming()方法返回一个迭代器 该迭代器中的每个元素都是一个TcpStream类型的实例
    // 每个TcpStream类型实例都代表一个TCP连接
    for stream in listener.incoming() {
        // 但是迭代器中产生的元素类型为 Result<TcpStream, E>类型的枚举 需要使用unwrap()方法来处理错误
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

/// 本函数用于从TCP连接中读取数据并打印请求相关信息
/// 通常来讲,读取操作是不需要mut的.但是TcpStream.read()方法需要一个可变引用(这里的可变引用指的是mut stream: TcpStream)
/// 这是因为TcpStream内部维护了一个缓冲区,每次读取数据时都会将数据写入到这个缓冲区中,因此需要一个可变引用来修改这个缓冲区
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = "GET / HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get.as_bytes()) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}