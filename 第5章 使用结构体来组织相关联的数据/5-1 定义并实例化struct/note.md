# PART1. 定义与实例化struct

## 1.1 定义struct

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

## 1.2 实例化struct

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: String::from("example@gmail.com"),
        username: String::from("example"),
        sign_in_count: 1,
        active: true,
    };
}
```

在初始化时,必须指定所有字段的值,否则会报错

# PART2. 取得struct中的某字段值

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: String::from("example@gmail.com"),
        username: String::from("example"),
        sign_in_count: 1,
        active: true,
    };

    println!("user1's email: {}", user1.email);
}
```

# PART3. 修改struct中的某字段值

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    // 一旦struct的实例是可变的,那么实例中的所有字段都是可变的
    let mut user1 = User {
        email: String::from("example@gmail.com"),
        username: String::from("example"),
        sign_in_count: 1,
        active: true,
    };

    user1.email = String::from("example1@gmail.com");
    println!("{}", user1.email);
}
```

注意:一旦struct的实例是可变的,那么实例中的所有字段都是可变的.Rust不允许只将struct的某个字段设置为可变

# PART4. 使用函数返回struct

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let email = String::from("example@gmail.com");
    let username = String::from("example");
    let mut user1 = build_user(email, username);
    
    println!("{}", user1.email);
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
```

# PART5. 字段初始化简写

当字段名与变量名相同时,可以使用字段初始化简写

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let email = String::from("example@gmail.com");
    let username = String::from("example");
    let mut user1 = build_user(email, username);

    println!("{}", user1.email);
}

fn build_user(email: String, username: String) -> User {
    User {
        // 字段名和变量名均为 email 此时可以简写
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```

# PART5. struct更新语法

当你想基于某个struct实例来创建一个新实例时,可使用struct更新语法:

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: String::from("example@gmail.com"),
        username: String::from("example"),
        sign_in_count: 1,
        active: true,
    };

    let user2 = User{
        email: String::from("another@gmail.com"),
        username: String::from("another"),
        // 表示其他字段的值和user1对应字段的值相同
        ..user1
    };
}
```

# PART6. Tuple Struct

Tuple Struct是一种特殊的struct,它的字段没有名字,只有类型

Tuple Struct整体是有类型的,但它里边的字段是没有名字的.也就是说从外部看它是一个struct,而从内部看它是一个tuple

使用场景:当你想给相同类型的元组赋予不同的类型时,可以使用Tuple Struct

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    // black和origin是不同的类型
    let origin = Point(0, 0, 0);
}
```

# PART7. Unit-Like Struct

Unit-Like Struct是一种特殊的struct,它没有字段,类似于空元组()

使用场景:当你需要在某个类型上实现某个trait,但是在这个类型中又没有需要存储的数据时,可以使用Unit-Like Struct

# PART8. struct的数据所有权

例:

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

在这个例子中,User实例的username和email字段都是String类型,而非&str类型,换言之User实例拥有的是username字段和email字段的数据所有权,而非引用

- 该struct实例拥有其所有数据
- 只要struct实例是有效的,那么其所有字段也是有效的

struct中也可以存放引用,但这需要使用生命周期参数,这将在后续章节中讲解

- 生命周期保证只要struct实例时有效的,那么struct中的引用也是有效的
- 如果struct中存储引用,而不使用生命周期,就会报错

例:

```rust
struct User {
    // struct中使用了引用但是没有生命周期参数
    email: &str, // error: expected lifetime parameter
    username: &str,
    sign_in_count: u64,
    active: bool,
}
```

```bash
cargo build
   Compiling lifecycle_struct v0.1.0 (/lifecycle_struct)
error[E0106]: missing lifetime specifier
 --> src/main.rs:3:12
  |
3 |     email: &str, // error: expected lifetime parameter
  |            ^ expected named lifetime parameter
  |
help: consider introducing a named lifetime parameter
  |
1 ~ struct User<'a> {
2 |     // struct中使用了引用但是没有生命周期参数
3 ~     email: &'a str, // error: expected lifetime parameter
  |

error[E0106]: missing lifetime specifier
 --> src/main.rs:4:15
  |
4 |     username: &str,
  |               ^ expected named lifetime parameter
  |
help: consider introducing a named lifetime parameter
  |
1 ~ struct User<'a> {
2 |     // struct中使用了引用但是没有生命周期参数
3 |     email: &str, // error: expected lifetime parameter
4 ~     username: &'a str,
  |

For more information about this error, try `rustc --explain E0106`.
error: could not compile `lifecycle_struct` (bin "lifecycle_struct") due to 2 previous errors
```