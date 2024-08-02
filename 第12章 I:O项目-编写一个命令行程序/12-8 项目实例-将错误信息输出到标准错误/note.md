# PART1. 将错误信息输出到标准错误

目前我们的实现是将错误信息和结果都打印到了终端上.但是这样会有一个问题:如果我们将结果重定向到文件,那么错误信息也会被重定向到文件中.这样就会导致错误信息和结果混在一起,不方便查看.

- 标准输出: stdout
  - `println!`: 将字符串打印到标准输出
- 标准错误: stderr
  - `eprintln!`: 将字符串打印到标准错误

这时我们先不着急改程序,先看看如果将程序的输出重定向到某个文件中,会发生什么情况:

```
cargo run > output.txt
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/minigrep`
```

```
cat output.txt 
Problem parsing arguments: not enough arguments
```

很明显,我们将错误信息也重定向到了文件中.这样就会导致错误信息和结果混在一起,不方便查看.

目前我们的实现中,所有的错误处理都集中在了`main()`函数中,因此集中修改即可:

```rust
use minigrep::{Config, run};
use std::{env, process};

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

再次测试:(注意测试前删除`output.txt`文件)

```
cargo run > output.txt
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/minigrep`
Problem parsing arguments: not enough arguments
```

```
cat output.txt 
 
```

可以看到,当出现错误时,文件中没有任何内容,且错误信息被打印到了终端上.

测试正常情况:

```
cargo run to poem.txt > output.txt
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/minigrep to poem.txt`
```

```
cat output.txt 
Search for to
In file poem.txt
Are you nobody, too?
How dreary to be somebody!
```