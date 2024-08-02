# PART1. 枚举

枚举允许我们列举所有可能的值来定义一个类型.枚举中所有可能的值称为枚举的变体

# PART2. 定义枚举

```rust
enum IpAddrKind {
    V4,
    V6,
}
```

# PART3. 使用枚举

```rust
enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
}

fn route(ip_type: IpAddrKind) {}
```

# PART4. 将数据附加到枚举的变体中

枚举作为struct中的字段的例子:

```rust
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}
```

Rust允许我们将任意类型的数据附加到枚举的变体中:

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
```

这样的优点在于:

- 不需要使用额外的struct
- 每个变体可以有不同的类型以及关联的数据字面量

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
}
```

# PART5. 标准库中的IpAddr

```rust
#[cfg_attr(not(test), rustc_diagnostic_item = "IpAddr")]
#[stable(feature = "ip_addr", since = "1.7.0")]
#[derive(Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum IpAddr {
    /// An IPv4 address.
    #[stable(feature = "ip_addr", since = "1.7.0")]
    V4(#[stable(feature = "ip_addr", since = "1.7.0")] Ipv4Addr),
    /// An IPv6 address.
    #[stable(feature = "ip_addr", since = "1.7.0")]
    V6(#[stable(feature = "ip_addr", since = "1.7.0")] Ipv6Addr),
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[stable(feature = "rust1", since = "1.0.0")]
pub struct Ipv4Addr {
    octets: [u8; 4],
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[stable(feature = "rust1", since = "1.0.0")]
pub struct Ipv6Addr {
    octets: [u8; 16],
}
```

从IpAddr的枚举类型中可以看到,IpAddr有两个变体,分别是V4和V6,而V4和V6分别是Ipv4Addr和Ipv6Addr的枚举类型.

# PART6. 变体不同类型

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    // 这里Write的类型是String 不是tuple
    // 如果想要tuple类型(只有1个String的tuple) 那么应该写成Write(String, )
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let q = Message::Quit;
    let m = Message::Move { x: 1, y: 2 };
    let w = Message::Write(String::from("hello"));
    let c = Message::ChangeColor(0, 0, 0);
}
```

# PART7. 为枚举定义方法

和为struct定义方法一样,同样是使用impl块为枚举定义方法

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    // 这里Write的类型是String 不是tuple
    // 如果想要tuple类型(只有1个String的tuple) 那么应该写成Write(String, )
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {

    }
}

fn main() {
    let q = Message::Quit;
    let m = Message::Move { x: 1, y: 2 };
    let w = Message::Write(String::from("hello"));
    let c = Message::ChangeColor(0, 0, 0);

    m.call();
}
```

