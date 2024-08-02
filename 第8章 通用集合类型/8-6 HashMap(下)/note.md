# PART1. 更新`HashMap<K, V>`

- HashMap大小可变
- 每个K同时只能对应一个V
- 更新HashMap中的数据
  - K已经存在并对应一个V:
    - 替换现有的V
    - 保留现有的V,忽略新的V
    - 合并现有的V和新的V
  - K不存在:
    - 添加新的KV

## 1.1 替换现有的V

如果向HashMap中插入一对KV,然后再插入同样的K,但是V不同,则原来的V会被替换掉:

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(String::from("a"), 1);
    map.insert(String::from("b"), 2);
    println!("{:?}", map);

    map.insert(String::from("a"), 3);
    println!("{:?}", map);
}
```

```bash
cargo run
   Compiling recover_hashmap v0.1.0 (/recover_hashmap)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.36s
     Running `target/debug/recover_hashmap`
{"a": 1, "b": 2}
{"a": 3, "b": 2}
```

## 1.2 只在K不对应任何值时插入V

换言之这个就是保留现有的V

`entry()`方法:检查制定的K是否对应一个V

- 参数:K
- 返回值:Entry枚举
  - 如果K对应一个V,返回Entry::Occupied
  - 如果K不对应任何V,返回Entry::Vacant

```rust
use std::collections::HashMap;
use std::collections::hash_map::Entry;

fn main() {
  let mut map = HashMap::new();

  map.insert(String::from("a"), 1);
  map.insert(String::from("b"), 2);

  let e = map.entry(String::from("c"));

  // match 也会获取到e的所有权 所以这里借用e的所有权 确保e不会发生移动
  match &e {
    // entry()方法的返回值为一个Entry枚举类型, 它有两个变体:Occupied和Vacant

    // Occupied表示这个K已经存在 它包含了这个K对应的V
    Entry::Occupied(o) => {
      println!("Occupied: {:?}", o);
    },

    // Vacant表示这个K不存在 它包含了这个K
    Entry::Vacant(v) => {
      println!("Vacant: {:?}", v);
    }
  }

  // or_insert()方法: 如果K存在,则返回对应的V的可变引用;
  // 如果K不存在,则插入K和V,并返回V的可变引用
  e.or_insert(3);
  println!("{:?}", map);

  let e2 = map.entry(String::from("a"));
  e2.or_insert(4);
  println!("{:?}", map);
}
```

```bash
cargo run
   Compiling retain_hashmap v0.1.0 (/retain_hashmap)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.40s
     Running `target/debug/retain_hashmap`
Vacant: VacantEntry("c")
{"c": 3, "a": 1, "b": 2}
{"c": 3, "a": 1, "b": 2}
```

## 1.3 基于现有的V来更新V

`or_insert()`方法返回一个对V的可变引用,这样就可以修改V了

```rust
use std::collections::HashMap;

fn main() {
    let words = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in words.split_whitespace() {
        // or_insert()方法返回的是一个可变引用 若K存在 则返回对应的V的可变引用
        // 若K不存在 则插入K-V并返回V的可变引用
        let count :&mut i32 = map.entry(word).or_insert(0);

        // 由于count是一个可变引用 所以想要修改它的值需要先解引用
        *count += 1;
    }

    println!("{:?}", map);
}
```

```bash
cargo run
   Compiling modify_hashmap v0.1.0 (/modify_hashmap)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.93s
     Running `target/debug/modify_hashmap`
{"hello": 1, "wonderful": 1, "world": 2}
```

# PART2. Hash函数

默认情况下,HashMap使用的是一个加密功能强大的Hash函数,可以抵抗拒绝服务攻击(DoS)

- 它不是最快的Hash函数
- 但它具有更好的安全性

我们可以指定不同的hasher来切换到另一个Hash函数

- hasher是实现了`BuildHasher`trait的类型
