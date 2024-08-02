use std::fmt;

// 为Result<T, std::io::Error>枚举定义别名
pub type MyResult<T> = Result<T, std::io::Error>;

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> MyResult<usize>;
    fn flush(&mut self) -> MyResult<()>;

    fn write_all(&mut self, buf: &[u8]) -> MyResult<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> MyResult<()>;
}

fn main() {
}
