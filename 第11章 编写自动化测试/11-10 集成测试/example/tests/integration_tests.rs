// 本文件针对src/lib.rs中的函数进行测试
// 也就是针对example package下的library crate进行测试
use example;

// 使用集成测试中的子模块
mod common;

// tests目录下的测试函数不需要写#[cfg(test)]
// 因为cargo会把tests目录下的文件做特殊处理:该目录下的文件只会在执行cargo test命令时才会进行编译,因此不需要再标注#[cfg(test)]了
#[test]
fn it_adds_two() {
    // 使用集成测试中的子模块中的函数
    common::setup();
    assert_eq!(4, example::add_two(2));
}