# PART1. 闭包的类型推断

- 闭包不要求标注参数和返回值的类型
  - 函数之所以要求标注参数和返回值类型,是因为函数是暴露给用户的显式接口的一部分,严格定义接口有助于所有人对参数和返回值类型取得共识
  - 但闭包并不会被用于暴露接口,闭包存储在一个变量中,在使用闭包时也不需要命名,闭包也不会暴露给代码库的用户,因此不强制要求闭包标注参数和返回值类型
  - 且闭包通常比较短小,只在狭小的上下文中工作,编译器通常能够推断出其参数和返回值的类型
- 同样可以手动为闭包添加类型标注

例:

```rust
let expensive_closure = |num: u32| -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};
```

# PART2. 函数和闭包的定义语法

```rust
fn main() {
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x| {x + 1};
    let add_one_v4 = |x| x + 1;
}

fn add_one_v1(x :u32) -> u32 {
    x + 1
}
```

# PART3. 闭包的类型推断

注意: 闭包的定义最终只会为参数/返回值推断出唯一具体的类型,如果闭包的参数和返回值类型无法推断出唯一的具体类型,则会报错

```rust
fn main() {
    // 仅定义闭包而不使用闭包的情况下 无法推断x的类型 会报错
    let example_closure = |x| x;

    // 若此时调用闭包 则可以推断x的类型 x的类型一旦被推断出来 就不能再改变
    let s = example_closure(String::from("hello"));

    // 此时再使用不同类型的参数调用闭包 会报错
    let n = example_closure(5);
}
```