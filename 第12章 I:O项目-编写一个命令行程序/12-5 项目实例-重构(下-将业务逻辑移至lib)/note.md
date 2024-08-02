# PART1. 将业务逻辑移至`lib.rs`

## 1.1 将业务逻辑封装为一个函数

我们先来看一下`main.rs`中关于读取文件的代码：

```rust
let content :String = fs::read_to_string(config.filename).
        expect("Something went wrong reading the file");
    println!("With text:\n{}", content);
```

这段代码中:

- 若读取文件成功则什么都不返回,仅仅是将文件中的内容打印出来
- 若读取文件失败则触发panic

这就意味着如果我们将这个过程封装成一个函数的话,它应该返回一个`Result`类型的值,而不是直接触发panic.

我们将这个函数命名为`run`:

```rust
use std::{env, fs, process};
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::new(args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Search for {}", config.query);
    println!("In file {}", config.filename);

    run(config);
}

// 此处仅知道Error表示实现了Error trait的类型即可
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content :String = fs::read_to_string(config.filename)?;
    println!("With text:\n{}", content);
    Ok(())
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

很明显,在调用`run()`函数之后,我们没有对其返回值进行处理.但是这和上一节中使用`Result.unwrap_or_else()`略有不同:此处我们不需要从`run()`函数的返回值中读取任何内容,仅当该函数返回`Err`变体时,才对其进行处理即可.

```rust
use std::{env, fs, process};
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::new(args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Search for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

// 此处仅知道Error表示实现了Error trait的类型即可
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content :String = fs::read_to_string(config.filename)?;
    println!("With text:\n{}", content);
    Ok(())
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

## 1.2 将业务逻辑移至`lib.rs`

工程结构如下:

```
 tree ./    
./
├── Cargo.lock
├── Cargo.toml
├── poem.txt
└── src
    ├── lib.rs
    └── main.rs

1 directory, 5 files
```

`src/lib.rs`:

```rust
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content :String = fs::read_to_string(config.filename)?;
    println!("With text:\n{}", content);
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: Vec<String>) -> Result<Config, &'static str> {
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

这样就可以对`lib.rs`中的API进行集成测试了.

`src/main.rs`:

```rust
use minigrep::{Config, run};
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::new(args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Search for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
```