# PART1. 程序目前存在的问题

再来看这段代码:

```rust
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query :String = args[1].clone();
    let filename :String = args[2].clone();

    println!("Search for {}", query);
    println!("In file {}", filename);

    let content :String = fs::read_to_string(filename).
        expect("Something went wrong reading the file");
    println!("With text:\n{}", content)
}
```

这段代码存在4个问题:

1. `main()`函数负责的功能太多了:
    - 既负责了参数解析
    - 又负责了读取文件
2. `query`、`filename`、`content`这几个变量太分散了,应该放在1个结构体中,让结构更清晰
3. 读取文件失败时打印的信息太模糊了,没法根据打印的信息来确认读取失败的原因
4. 程序中对错误处理的代码应该统一放置在一处

# PART2. 二进制程序关注点分离的指导性原则

- 将程序拆分为`main.rs`和`lib.rs`,把业务逻辑放在`lib.rs`中
- 当命令行解析逻辑较少时,放在`main.rs`中即可
- 当命令行解析逻辑较复杂时,需要将这些代码从`main.rs`中转移到`lib.rs`中

# PART3. `main()`函数的功能

经过上述拆分,留在`main()`函数中的功能有:

- 使用参数值调用命令行解析逻辑
- 进行其他配置
- 调用`lib.rs`中的`run()`函数
- 处理`run()`函数可能出现的错误

# PART4. 重构-解析参数部分

```rust
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, filename) = parse_config(args);

    println!("Search for {}", query);
    println!("In file {}", filename);

    let content :String = fs::read_to_string(filename).
        expect("Something went wrong reading the file");
    println!("With text:\n{}", content)
}

fn parse_config(args: Vec<String>) -> (String, String) {
    let query = args[1].clone();
    let filename = args[2].clone();
    (query, filename)
}
```

此时还有问题,`query`和`filename`,在返回之后,仍旧需要2个变量接收.很明显我们使用元组这个结构是不合理的,因为这两个变量有比使用元组存储更紧密的联系.

比元组更紧密的数据结构,自然是定义一个结构体了.

```rust
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = parse_config(args);

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

fn parse_config(args: Vec<String>) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();
    Config {
        query,
        filename,
    }
}
```

现在还有一个问题:`parse_config()`很明显是一个初始化`Config`结构体实例的过程.所以这个函数应该是`Config`结构体的关联函数:

```rust
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::new(args);

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
    fn new(args: Vec<String>) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();
        Config {
            query,
            filename,
        }
    }
}
```