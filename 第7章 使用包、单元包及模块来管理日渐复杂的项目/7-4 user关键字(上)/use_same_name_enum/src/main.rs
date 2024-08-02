// 由于有同名的语法项,因此在引入时,引入到同名语法项的父级(本例中就是模块)
use std::fmt;
use std::io;

fn main() {
    println!("Hello, world!");
}

// 使用时,使用 `父级模块::语法项` 的方式
fn f1() -> fmt::Result {
    Ok(())
}

// 使用时,使用 `父级模块::语法项` 的方式
fn f2() -> io::Result<()> {
    Ok(())
}
