# PART1. 本章目标

实现一个类似`grep`的命令行工具，支持以下功能:

- 在指定的文件中搜索出指定的文字
- 该工具接收一个文件名和一个字符串作为参数
- 在读取文件内容时,搜索包含指定字符串的行,并将这些行打印出来
- 假设该工具名称为`minigrep`,则调用方式为`cargo run searchstring filename`
  - 其中`searchstring`为要搜索的字符串
  - `filename`为要搜索的文件名

## 1.1 读取命令行参数

- `std::env::args()`:返回一个迭代器.可以调用迭代器的`collect()`方法将其转换为一个`Vec<String>`

```rust
use std::env;

fn main() {
    // 此处必须指定args的类型 因为collect()方法返回的类型为一个 实现了 FromIterator trait 的泛型
    // 因此编译器无法推断其具型 必须手动指定
    // 需要注意的是 env::args()函数只能接收Unicode字符
    // 若想要能够接收非法的Unicode字符 可以使用std::env::args_os()函数
    let args: Vec<String> = env::args().collect();

    // 第1个参数为要搜索的内容
    let query :String = args[1].clone();
    
    // 第2个参数为要搜索的文件
    let filename :String = args[2].clone();

    println!("Search for {}", query);
    println!("In file {}", filename);
}
```

