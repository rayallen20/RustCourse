use std::error::Error;
use std::fs::File;

// Box<dyn Error> 是一个trait对象, 在这里简单的理解为是任何可能的错误类型即可
fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;
    Ok(())
}
