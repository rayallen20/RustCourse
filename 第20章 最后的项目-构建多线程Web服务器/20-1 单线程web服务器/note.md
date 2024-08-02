# PART1. 需求

本章主要任务为构建一个多线程的web服务器,要求有以下功能:

- 在socket上监听TCP连接
- 解析少量HTTP请求
- 创建一个合适的HTTP响应
- 使用线程池改进服务器的吞吐量

# PART2. 单线程web服务器的实现

## 2.1 监听TCP连接

```
cargo new my_web_server
    Creating binary (application) `my_web_server` package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
```

```rust
use std::net::TcpListener;

fn main() {
    // 创建一个TcpListener并绑定到本地地址
    // TcpListener::bind()关联函数会返回一个Result<T, E>类型的枚举 此处我们简单使用unwrap()方法来处理错误即可
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // listener.incoming()方法返回一个迭代器 该迭代器中的每个元素都是一个TcpStream类型的实例
    // 每个TcpStream类型实例都代表一个TCP连接
    for stream in listener.incoming() {
        // 但是迭代器中产生的元素类型为 Result<TcpStream, E>类型的枚举 需要使用unwrap()方法来处理错误
        let stream = stream.unwrap();
        println!("Connection established!");
    }
}
```

```
cargo run
   Compiling my_web_server v0.1.0 (//my_web_server)
...
warning: `my_web_server` (bin "my_web_server") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.73s
     Running `target/debug/my_web_server`
```

使用浏览器访问`http://127.0.0.1:7878`,控制台输出:

```
Connection established!
Connection established!
Connection established!
Connection established!
Connection established!
Connection established!
Connection established!
```

## 2.2 读取请求

```rust
use std::io::Read;
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
    // 声明一个512字节大小的缓冲区 这个大小对于简单的HTTP请求来说已经够用了
    let mut buffer = [0; 512];
    // 从TcpStream中读取数据并将其写入到缓冲区中
    stream.read(&mut buffer).unwrap();
    // String::from_utf8_lossy()函数用于将字节数组(即[u8])转换为字符串
    // 若字节数组中包含无效的UTF-8字符,则该函数会用�字符(U+FFFD)替换这些无效字符
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}
```

```
cargo run
   Compiling my_web_server v0.1.0 (/my_web_server)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 9.02s
     Running `target/debug/my_web_server`
```

使用浏览器访问`http://127.0.0.1:7878`,控制台输出:

```
Request: GET / HTTP/1.1
Host: 127.0.0.1:7878
Connection: keep-alive
Cache-Control: max-age=0
sec-ch-ua: "Not/A)Brand";v="8", "Chromium";v="126", "Google Chrome";v="126"
sec-ch-ua-mobile: ?0
sec-ch-ua-platform: "macOS"
Upgrade-Insecure-Requests: 1
User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=
...(由于浏览器没有收到响应,因此会一直请求,控制台输出会一直打印这段内容)
```

## 2.3 观察HTTP请求

请求格式基本如下:

```
Method Request-URI HTTP-Version CRLF
headers CRLF
message-body
```

### 2.3.1 `Method Request-URI HTTP-Version CRLF`

这一行对应例子中的`Request: GET / HTTP/1.1`

- `GET`: 描述请求方式
- `/`: 请求的资源路径(uri)
- `HTTP/1.1`: HTTP协议版本
- `CRLF`: 这个符号在例子中没有显示,但是实际上是存在的.它表示回车换行符(`\r\n`)
  - `CR`: Carriage Return, 回车符(`\r`)
  - `LF`: Line Feed, 换行符(`\n`)

### 2.3.2 `headers CRLF`

这一行对应例子中的:

```
Host: 127.0.0.1:7878
Connection: keep-alive
Cache-Control: max-age=0
sec-ch-ua: "Not/A)Brand";v="8", "Chromium";v="126", "Google Chrome";v="126"
sec-ch-ua-mobile: ?0
sec-ch-ua-platform: "macOS"
Upgrade-Insecure-Requests: 1
User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=
```

部分

### 2.3.3 `message-body`

本例中我们没有发送请求体,因此这一部分为空

## 2.4 编写响应

### 2.4.1 响应格式

```
HTTP-Version Status-Code Reason-Phrase CRLF
headers CRLF
message-body
```

一个使用了HTTP 1.1版本,状态码为200,原因短语为OK,没有消息头与消息体的响应内容如下:

```
HTTP/1.1 200 OK
```

### 2.4.2 编写响应

```rust
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
    // 声明一个512字节大小的缓冲区 这个大小对于简单的HTTP请求来说已经够用了
    let mut buffer = [0; 512];
    // 从TcpStream中读取数据并将其写入到缓冲区中
    stream.read(&mut buffer).unwrap();
    // String::from_utf8_lossy()函数用于将字节数组(即[u8])转换为字符串
    // 若字节数组中包含无效的UTF-8字符,则该函数会用�字符(U+FFFD)替换这些无效字符
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    // 定义HTTP响应
    let response = "HTTP/1.1 200 OK\r\n\r\n";

    // 将HTTP响应写入到TcpStream中
    stream.write(response.as_bytes()).unwrap();

    // 将TcpStream中的数据刷新到底层的TCP连接中
    stream.flush().unwrap();
}
```

这次再请求`http://127.0.0.1:7878`,浏览器将不会显示无法响应,而是显示一个空白页面

## 2.5 返回HTML

### 2.5.1 定义HTML内容

`my_web_server/hello.html`:

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Hello!</title>
</head>
<body>
    <p>Hi from Rust</p>
</body>
</html>
```

### 2.5.2 读取HTML内容并作为响应体返回

这一步就比较简单了,读取`hello.html`的内容并将其作为响应体返回即可

```rust
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
    // 声明一个512字节大小的缓冲区 这个大小对于简单的HTTP请求来说已经够用了
    let mut buffer = [0; 512];
    // 从TcpStream中读取数据并将其写入到缓冲区中
    stream.read(&mut buffer).unwrap();
    // String::from_utf8_lossy()函数用于将字节数组(即[u8])转换为字符串
    // 若字节数组中包含无效的UTF-8字符,则该函数会用�字符(U+FFFD)替换这些无效字符
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    // 读取hello.html文件的内容
    let contents = fs::read_to_string("hello.html").unwrap();

    // 定义HTTP响应(将hello.html文件的内容作为响应体)
    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

    // 将HTTP响应写入到TcpStream中
    stream.write(response.as_bytes()).unwrap();

    // 将TcpStream中的数据刷新到底层的TCP连接中
    stream.flush().unwrap();
}
```

此时再请求`http://127.0.0.1:7878`,就可以看到浏览器中显示的`hello.html`的内容了

## 2.6 验证请求有效性并选择性地响应

需求:

- 当请求方式为`GET`且请求`uri`为`/`时,返回`hello.html`的内容
- 其他情况下,响应状态码为404,且返回一个错误页面

### 2.6.1 定义错误页面

`src/404.html`

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Hello</title>
</head>
<body>
  <h1>Oops!</h1>
  <p>Sorry, I don't know what you're asking for.</p>
</body>
</html>
```

### 2.6.2 判断请求uri并返回相应内容

```rust
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
    // 若请求中包含GET / 则返回hello.html的内容
    if buffer.starts_with(get.as_bytes()) {
        let contents = fs::read_to_string("hello.html").unwrap();
        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        // 否则响应码为404 且 返回404.html的内容
        let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        let contents = fs::read_to_string("404.html").unwrap();
        let response = format!("{}{}", status_line, contents);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
```

## 2.7 重构

消除if else代码块中的重复代码即可:

```rust
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
```