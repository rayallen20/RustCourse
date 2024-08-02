# PART1. 使用环境变量

这里我们想通过环境变量来控制我们的项目的一些配置, 比如数据库的连接地址,端口号等等.

在本项目中,我们使用环境变量来区分匹配的内容是否区分大小写.例如:

- 打开该环境变量时,匹配的内容不区分大小写:搜索`duck`时能够匹配出`Duck`、`DUCK`等
- 关闭该环境变量时,匹配的内容区分大小写:搜索`duck`时只能匹配出`duck`

# PART2. 使用TDD开发功能

## 2.1 编写测试用例

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // 大小写敏感的测试用例
    fn case_sensitive() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }

    #[test]
    // 大小写不敏感的测试用例
    fn case_insensitive() {
        let query = "rUsT";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, content));
    }
}
```

## 2.2 编写函数

还是和之前一样,先写一个`search_case_insensitive()`函数的声明:

```rust
pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    vec![]
}
```

## 2.3 测试

```
cargo test
...
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.08s
     Running unittests src/lib.rs (target/debug/deps/minigrep-46eec06a0a17d646)

running 2 tests
test tests::case_sensitive ... ok
test tests::case_insensitive ... FAILED

failures:

---- tests::case_insensitive stdout ----
thread 'tests::case_insensitive' panicked at src/lib.rs:76:9:
assertion `left == right` failed
  left: ["Rust:", "Trust me."]
 right: []
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::case_insensitive

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--lib`
```

如同预期的,测试失败了.现在就来实现该函数的逻辑,使得测试通过.

## 2.4 实现函数

实现的思路也比较简单:将待匹配的内容和查询的内容都转换为小写,然后进行匹配即可

```rust
pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();
    let lower_query: String = query.to_lowercase();

    for line in content.lines() {
        if line.to_lowercase().contains(&lower_query) {
            results.push(line);
        }
    }

    results
}
```

## 2.5 测试

```
cargo test
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/lib.rs (target/debug/deps/minigrep-46eec06a0a17d646)

running 2 tests
test tests::case_insensitive ... ok
test tests::case_sensitive ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/minigrep-5ff5a5b93151e998)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests minigrep

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

# PART3. 添加是否区分大小写的功能

## 3.1 为`Config`结构体添加用于标识是否区分大小写的字段

```rust
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}
```

此时`Config::new()`关联函数会有编译错误,我们暂时先不管

## 3.2 `run()`函数中实现根据`case_sensitive`字段来选择调用哪个搜索函数

```rust
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content :String = fs::read_to_string(config.filename)?;

    let results: Vec<&str> = if config.case_sensitive {
        search(&config.query, &content)
    } else {
        search_case_insensitive(&config.query, &content)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}
```

## 3.3 `Config::new()`关联函数中解析环境变量

- `env::var()`函数:获取环境变量的值.该函数返回一个`Result<String, VarError>`,若环境变量不存在,则返回`Err`变体;否则返回`Ok`变体,其中包含环境变量的值
- `Result.is_err()`方法:检查`Result`是否是`Err`变体.若是`Err`变体,则返回`true`;否则返回`false`

这里我们人为认定:

- 若环境变量`CASE_INSENSITIVE`存在,则表示区分大小写
- 若环境变量`CASE_INSENSITIVE`不存在,则表示不区分大小写

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

# PART4. 测试

## 4.1 测试区分大小写的情况

只需不设置环境变量即可

```
 cargo run to poem.txt
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/minigrep to poem.txt`
Search for to
In file poem.txt
Are you nobody, too?
How dreary to be somebody!
```

## 4.2 测试不区分大小写的情况

```
CASE_INSENSITIVE=1 cargo run to poem.txt
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/minigrep to poem.txt`
Search for to
In file poem.txt
Are you nobody, too?
How dreary to be somebody!
To tell your name the livelong day
To an admiring bog!
```

TODO:

1. 将配置开关设置为命令行参数
2. 确定命令行参数与环境变量的优先级