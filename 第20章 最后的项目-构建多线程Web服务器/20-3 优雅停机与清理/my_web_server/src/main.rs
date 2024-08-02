use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;
use my_web_server::pool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = pool::ThreadPool::new(4);
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        pool.execute(||{
            handle_connection(stream);
        })
    }

    println!("Shutting down.");
}

/// 本函数用于从TCP连接中读取数据并打印请求相关信息
/// 通常来讲,读取操作是不需要mut的.但是TcpStream.read()方法需要一个可变引用(这里的可变引用指的是mut stream: TcpStream)
/// 这是因为TcpStream内部维护了一个缓冲区,每次读取数据时都会将数据写入到这个缓冲区中,因此需要一个可变引用来修改这个缓冲区
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = "GET / HTTP/1.1\r\n";
    let sleep = "GET /sleep HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get.as_bytes()) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep.as_bytes()) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    }
    else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}