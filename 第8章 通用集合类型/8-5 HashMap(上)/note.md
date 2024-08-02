# PART1. `HashMap<K, V>`

HashMap也涉及到泛型,它有2个泛型参数:

- `K`: 键的类型
- `V`: 值的类型

`Hash()`函数决定了如何在内存中存储键值对

# PART2. 创建HashMap

## 2.1 `HashMap::new()`

```rust
use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:?}", scores);
}
```

```bash
cargo run
   Compiling create_hashmap v0.1.0 (/create_hashmap)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.82s
     Running `target/debug/create_hashmap`
{"Blue": 10, "Yellow": 50}
```

- HashMap用的比较少,不在Prelude中,所以需要`use std::collections::HashMap;`
- 标准库对它的支持比较少,没有内置的宏来创建HashMap
  - 不像`vec!`宏,可以创建一个Vec
- 数据也是存储在Heap上的
- HashMap是同构的:
  - 所有的K必须是同一种类型
  - 所有的V也必须是同一种类型

## 2.2 `collect()`

```rust
use std::collections::HashMap;

fn main() {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    // zip() 方法创建一个元组的迭代器,其中新的迭代器将同时包含 teams 和 initial_scores 中的元素
    let team_scores_tuple = teams.iter().zip(initial_scores.iter());
    let team_scores_tuple_clone = team_scores_tuple.clone();

    // 在Rust中 迭代器是一种消费者
    // 遍历迭代器的操作会"消费"迭代器中的元素,这意味着迭代器会获取元素的所有权并在遍历过程中将其移出
    // 这里就将 team_scores_tuple 这个迭代器中的所有元素全部移出了
    // 因此这里需要事前准备一个克隆 用于后续的操作
    for ele in team_scores_tuple {
        println!("{:?}", ele); // ("Blue", 10) ("Yellow", 50)
    }

    // 使用 collect() 方法将元组转换为 HashMap
    // collect()方法可以将数据转换为多种不同的集合类型 所以需要指定类型
    // 这里的_是占位符,表示HashMap的键和值的类型是由collect()方法推断出来的
    let team_scores: HashMap<_, _> = team_scores_tuple_clone.collect();
    println!("{:?}", team_scores); // {"Blue": 10, "Yellow": 50}
}
```

```bash
cargo run
   Compiling collect_hashmap v0.1.0 (/collect_hashmap)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.78s
     Running `target/debug/collect_hashmap`
("Blue", 10)
("Yellow", 50)
{"Blue": 10, "Yellow": 50}
```

# PART3. HashMap和所有权

- 对于实现了Copy trait的类型(如i32),值会被复制到HashMap中
- 对于拥有所有权的值(如String),值会被移动,但是所有权会转移给HashMap

```rust
use std::collections::HashMap;

fn main() {
    let field_name = String::from("Favorite color");
    let field_value = 50;

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    println!("{:?}", map);
    println!("{:?}", field_name); // error: value borrowed here after move
    println!("{:?}", field_value);
}
```

error[E0382]: borrow of moved value: `field_name`

- 如果将值的引用插入HashMap,值本身不会被移动
  - 但是引用指向的值必须在HashMap有效时保持有效 

```rust
use std::collections::HashMap;

fn main() {
    let field_name = String::from("Favorite color");
    let field_value = 50;

    let mut map = HashMap::new();
    map.insert(&field_name, &field_value);

    println!("{:?}", map);
    println!("{:?}", field_name);
    println!("{:?}", field_value);
}
```

```bash
cargo run
   Compiling reference_hashmap v0.1.0 (/reference_hashmap)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.77s
     Running `target/debug/reference_hashmap`
{"Favorite color": 50}
"Favorite color"
50
```

# PART4. 访问HashMap中的值

`get()`方法:

- 参数是K的引用
- 返回一个Option<&V>

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();

    map.insert(String::from("a"), 1);
    map.insert(String::from("b"), 2);

    let exist_key = String::from("a");
    // 注意: get()方法接收的参数类型为&K, 返回值类型为Option<&V>
    let exist = map.get(&exist_key);

    match exist {
        Some(v) => println!("exist key: {} value: {}", exist_key, v),
        None => println!("not exist key: {}", exist_key),
    }
}
```

```bash
cargo run
   Compiling get_hashmap v0.1.0 (/get_hashmap)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.47s
     Running `target/debug/get_hashmap`
exist key: a value: 1
```

# PART5. 遍历HashMap

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(String::from("one"), 1);
    map.insert(String::from("two"), 2);

    // 这里如果不加上& 后续就不能再使用这个map了 因为它的所有权发生了移动
    for (key, value) in &map {
        println!("{}: {}", key, value);
    }

    // 加了&后就可以继续使用map
    println!("{:?}", map); // error: borrow of moved value: `map`
}
```

```bash
cargo run
   Compiling traverse_hashmap v0.1.0 (/traverse_hashmap)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.50s
     Running `target/debug/traverse_hashmap`
one: 1
two: 2
{"one": 1, "two": 2}
```