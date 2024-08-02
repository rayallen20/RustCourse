# PART1. 使用enum来存储多种数据类型

Vector只能存储相同类型的数据,而我们有时候需要存储不同类型的数据,这时候我们可以使用enum来存储多种数据类型.

```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let v: Vec<SpreadsheetCell> = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];

    for cell in &v {
        match cell {
            SpreadsheetCell::Int(value) => println!("Int: {}", value),
            SpreadsheetCell::Float(value) => println!("Float: {}", value),
            SpreadsheetCell::Text(value) => println!("Text: {}", value),
        }
    }
}
```

```bash
cargo run
   Compiling vector_and_enum v0.1.0 (/vector_and_enum)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.29s
     Running `target/debug/vector_and_enum`
Int: 3
Float: 10.12
Text: blue
```

Tips:如果想使用`println!`打印enum的值,需要实现`std::fmt::Display` trait.

```rust
#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let v: Vec<SpreadsheetCell> = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];

    println!("{:?}", v);
}
```

```bash
cargo run
   Compiling vector_and_enum v0.1.0 (/vector_and_enum)
...
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.38s
     Running `target/debug/vector_and_enum`
[Int(3), Float(10.12), Text("blue")]
```

Rust之所以在编译时就要确定Vector的类型,是为了确定在Heap上分配多少内存.

如果Vector允许存放各种类型,那么有些操作对于某些类型是合法的,而这些操作对于另外的类型可能就是非法的.

而类似例子中,使用enum来存储各种可能的类型,就可以避免这个问题.因为enum的每个变体都是确定的类型,所以在编译时就可以确定Vector的类型.