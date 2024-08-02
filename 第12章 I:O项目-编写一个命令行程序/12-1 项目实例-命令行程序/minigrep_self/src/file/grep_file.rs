use std::fs::File;
use std::io::{self, BufRead, BufReader};
pub struct GrepFile {
    // name: 文件名
    pub name: String,
    // contents: 按行存储的文件内容
    contents: Vec<String>,
    // match_lines: 包含指定内容的行号
    match_lines: Vec<i32>,
}

impl GrepFile {
    // 本方法用于根据给定的文件名,创建一个新的File实例
    pub fn new(name: &str) -> GrepFile {
        GrepFile {
            name: name.to_string(),
            contents: Vec::new(),
            match_lines: Vec::new(),
        }
    }

    // 本方法用于按行读取文件内容,并将其存储到contents字段中
    pub fn read(&mut self) -> io::Result<()> {
        let opening_file = File::open(&self.name)?;

        let reader = BufReader::new(opening_file);

        self.contents.clear();

        // 将每一行的内容存储到contents字段中
        for (_, line) in reader.lines().enumerate() {
            let content = line?;
            self.contents.push(content.clone());
        }

        Ok(())
    }

    // 本方法用于匹配文件内容中包含指定内容的行,并将行号存储到match_lines字段中
    pub fn match_lines(&mut self, pattern: &str) {
        self.match_lines.clear();
        for (i, line) in self.contents.iter().enumerate() {
            if line.contains(pattern) {
                self.match_lines.push((i+1) as i32);
            }
        }
    }

    // 本方法获取打印匹配的行号
    pub fn get_match_lines(&self) -> Vec<i32> {
        self.match_lines.clone()
    }
}