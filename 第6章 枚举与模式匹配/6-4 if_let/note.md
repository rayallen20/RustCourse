# PART1. if let

一个简单的控制流结构,用于处理:只关心一种匹配而忽略其他匹配的情况,等效于match语句中只匹配一个模式的情况.这也就意味着放弃了穷举的可能性

```rust
fn main() {
    let v :Some(u8)  = Some(0);

    // 仅当v是Some(42)时 才会执行println!语句
    if let v = Some(42) {
        println!("v: {}", v);
    }

    // if let与只匹配1个模式的match等效
    match v {
        Some(42) => println!("v: {}", 42),
        _ => (),
    }
}
```

# PART2. if let else

```rust
fn main() {
    let v :Some(u8)  = Some(0);

    // 仅当v是Some(42)时 进行特殊处理
    // 否则执行else分支
    if let v = Some(42) {
        println!("v: {}", v);
    } else {
        println!("other");
    }

    // 等效于match表达式中 仅匹配1个模式并使用通配符匹配其他模式
    match v {
        Some(42) => println!("v: {}", 42),
        _ => println!("other"),
    }
}
```