# PART1. 使用闭包和迭代器改进minigrep

## 1.1 使用迭代器优化`Config::new()`

原代码如下:

```rust
impl Config {
    pub fn new(args: Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        let config: Config = Config {
            query,
            filename,
            case_sensitive,
        };

        Ok(config)
    }
}
```

这里使用了`clone()`方法来复制`args`中的元素,但是这样会导致性能问题,因为`clone()`方法会复制整个字符串,而不是引用.

可以考虑使用迭代器作为该关联函数的参数,以便在函数内获得元素的所有权

另外,还可以使用迭代器的方法来完成长度检查和索引访问操作,使得函数的功能更加明确

## 1.2 修改`main()`函数

原代码如下:

```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::new(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Search for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
```

- `env::args()`函数的返回值类型本身就是Iterator Trait的实现,所以根据刚才的思路,可以直接把它传递给`Config::new()`

```rust
use minigrep::{Config, run};
use std::{env, process};
use std::env::Args;

fn main() {
    let args: Args = env::args();

    let config: Config = Config::new(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Search for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
```

## 1.3 修改`Config::new()`关联函数

```rust
impl Config {
    // 此处使用迭代器 由于需要在函数体内调用next()方法 所以需要使用mut
    pub fn new(mut args: Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        // 跳过第1个参数
        args.next();

        // 取第2和第3个参数
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        let config: Config = Config {
            query,
            filename,
            case_sensitive,
        };

        Ok(config)
    }
}
```

## 1.4 使用迭代器适配器方法优化`search()`函数

原代码如下:

```rust
pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}
```

这里使用迭代器适配器方法`filter()`和`collect()`来优化`search()`函数

```rust
pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let filter_closure = |line: &&str| -> bool {
        line.contains(query)
    };

    content.lines().filter(filter_closure).collect()
}
```

1. 减少了中间变量的使用
2. 使得代码更加简洁
3. 消除`mut results`这个可变状态,使得该函数的并行化更加容易

## 1.5 使用同样的方式修改`search_case_insensitive()`函数

```rust
pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let lower_query: String = query.to_lowercase();

    let filter_closure = |line: &&str| -> bool {
        line.to_lowercase().contains(&lower_query)
    };
    
    content.lines().filter(filter_closure).collect()
}
```

