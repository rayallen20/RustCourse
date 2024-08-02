# PART1. 重构-错误处理

问题:

`Config::new()`中,很明显没有处理错误:

```rust
let query = args[1].clone();
let filename = args[2].clone();
```

如果运行时没有传入任何参数,那么`args[1]`和`args[2]`就会越界,导致程序崩溃.

```rust
struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: Vec<String>) -> Config {
        if args.len() < 3 {
            panic!("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        Config {
            query,
            filename,
        }
    }
}
```

再次运行:

```
cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/minigrep`
thread 'main' panicked at src/main.rs:25:13:
not enough arguments
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

虽然错误信息更加友好,但是输出仍然有一些其他的残留信息,例如:`thread 'main' panicked at src/main.rs:25:13:`之类的信息.

另外,从代码上来看,由于`Config::new()`这个关联函数是有可能出现错误的,因此应该使用`Result`来处理错误.

```rust
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::new(args).unwrap();

    println!("Search for {}", config.query);
    println!("In file {}", config.filename);

    let content :String = fs::read_to_string(config.filename).
        expect("Something went wrong reading the file");
    println!("With text:\n{}", content)
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let config: Config = Config {
            query,
            filename
        };

        Ok(config)
    }
}
```

再次尝试运行:

```
cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/minigrep`
thread 'main' panicked at src/main.rs:7:44:
called `Result::unwrap()` on an `Err` value: "not enough arguments"
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

这次的错误信息更加友好,但是仍然有一些残留信息.

- `Result.unwrap_or_else()`方法: 该方法接收一个闭包作为参数.若`Result`的值为`Ok`变体,则返回`Ok`中的值;若`Result`的值为`Err`变体,则调用闭包函数.

此处我们只需在这个闭包中把错误信息打印出来,然后终止程序即可:

```rust
use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::new(args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Search for {}", config.query);
    println!("In file {}", config.filename);

    let content :String = fs::read_to_string(config.filename).
        expect("Something went wrong reading the file");
    println!("With text:\n{}", content)
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let config: Config = Config {
            query,
            filename
        };

        Ok(config)
    }
}
```

```
cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/minigrep`
Problem parsing arguments: not enough arguments
```

可以看到,此时的错误信息已经非常友好了.