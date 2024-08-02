use std::env;
use minigrep_self::file::grep_file::GrepFile;

fn main() {
    // 读取命令行参数
    let args: Vec<String> = env::args().collect();

    // 确认参数个数 此处要求有2个参数 因此args长度应该为3
    if args.len() != 3 {
        eprintln!("Usage: {} <pattern> <filename>", args[0]);
        std::process::exit(1);
    }

    let pattern: String = args[1].clone();
    let filename: String = args[2].clone();

    // 创建一个新的GrepFile实例
    let mut grep_file: GrepFile = GrepFile::new(&filename);

    // 读取文件内容
    grep_file.read().unwrap();
    grep_file.match_lines(&pattern);
    let match_lines: Vec<i32> = grep_file.get_match_lines();
    for match_line in &match_lines {
        println!("{}", match_line);
    }
}
