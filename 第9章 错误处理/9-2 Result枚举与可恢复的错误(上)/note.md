# PART1. Result枚举

```rust
#[derive(Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
#[must_use = "this `Result` may be an `Err` variant, which should be handled"]
#[rustc_diagnostic_item = "Result"]
#[stable(feature = "rust1", since = "1.0.0")]
pub enum Result<T, E> {
    /// Contains the success value
    #[lang = "Ok"]
    #[stable(feature = "rust1", since = "1.0.0")]
    Ok(#[stable(feature = "rust1", since = "1.0.0")] T),

    /// Contains the error value
    #[lang = "Err"]
    #[stable(feature = "rust1", since = "1.0.0")]
    Err(#[stable(feature = "rust1", since = "1.0.0")] E),
}
```

Result枚举有两个变体:

- T: 操作成功的情况下,OK变体中包含的值的类型
- E: 操作失败的情况下,Err变体中包含的值的类型

# PART2. 处理Result的一种方式: match表达式

和Option枚举一样,Result枚举及其变体也是由prelude(预导入模块)带入作用域的,所以可以直接使用Ok和Err变体.

```rust
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
```

```bash
cargo run              
warning: unused variable: `opening_file`
 --> src/main.rs:6:9
  |
6 |     let opening_file = match f {
  |         ^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_opening_file`
  |
  = note: `#[warn(unused_variables)]` on by default

warning: `match_result` (bin "match_result") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/match_result`
thread 'main' panicked at src/main.rs:12:13:
There was a problem opening the file: Os { code: 2, kind: NotFound, message: "No such file or directory" }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

There was a problem opening the file: Os { code: 2, kind: NotFound, message: "No such file or directory" }: 这一句是我们自定义的错误信息.

这里需要说明一点:

Result是一个泛型枚举,它的两个变体Ok和Err都有一个泛型参数,分别是T和E.本例中,`File::open()`函数返回的Result枚举是一个具型,其类型为`io::Result<File>`

```rust
#[stable(feature = "rust1", since = "1.0.0")]
pub type Result<T> = result::Result<T, Error>;
```

这里的`T`是`File`类型,`E`是`io::Error`类型.所以实际上`File::open()`函数返回的是`Result<File, io::Error>`类型的枚举.

# PART3. 匹配不同的错误

```rust
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
```

程序执行后:

```
tree ./ -L 1
./
├── Cargo.lock
├── Cargo.toml
├── hello.txt
├── src
└── target

2 directories, 3 files
```

- `Err(error) => match error.kind()`中的error: 这个error是`io::Error`类型的变量,该类型有一个`kind()`方法
- `error.kind()`:`error.kind()`方法返回一个`io::ErrorKind`枚举,这个枚举表示了不同的I/O错误类型
  - 本例中只处理了`ErrorKind::NotFound`这一种情况,其他情况都会直接panic.
- `ErrorKind::NotFound => match File::create("hello.txt")`: 如果文件不存在,则创建文件
  - 但是创建文件也可能失败,所以这里又嵌套了一个`match`表达式

这个例子中可以看出,`match`表达式虽然很有用,但是它太原始了.Result枚举有很多方法:

- 这些方法接收闭包作为参数
- 这些方法都是使用`match`表达式来实现的
- 使用这些方法,会让代码更加整洁

这些方法这里先不讲,但是给出一个例子:

- `Result.unwrap_or_else()`: 如果Result是`Ok`变体,则返回`Ok`中的值;如果是`Err`变体,则执行闭包并返回闭包的返回值

```rust
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
```

这段代码和之前的代码是等效的,但是更加简洁.

# PART4. `Result.unwrap()`

`Result.unwrap()`方法是`Result.unwrap_or_else()`方法的简写,它会直接返回`Ok`中的值,如果是`Err`变体,则会直接panic.

```rust
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
```

# PART5. `Result.expect()`

`Result.expect()`方法和`Result.unwrap()`方法类似,但是`Result.expect()`方法允许我们指定一个自定义的错误信息.

```rust
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
```