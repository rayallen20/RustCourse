// 由于有同名的语法项,因此在引入时,引入到enum,然后给同名的语法项起别名
use std::fmt::Result as FmtResult;
use std::io::Result as IoResult;

fn main() {
    println!("Hello, world!");
}

fn f1() -> FmtResult {
    Ok(())
}

fn f2() -> IoResult<()> {
    Ok(())
}
