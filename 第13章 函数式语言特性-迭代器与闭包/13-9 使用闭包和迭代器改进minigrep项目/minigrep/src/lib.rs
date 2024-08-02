use std::error::Error;
use std::{env, fs};
use std::env::Args;

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

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

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

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let filter_closure = |line: &&str| -> bool {
        line.contains(query)
    };

    content.lines().filter(filter_closure).collect()
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let lower_query: String = query.to_lowercase();

    let filter_closure = |line: &&str| -> bool {
        line.to_lowercase().contains(&lower_query)
    };

    content.lines().filter(filter_closure).collect()
}

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