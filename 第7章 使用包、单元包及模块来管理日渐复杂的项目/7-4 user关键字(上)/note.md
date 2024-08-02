# PART1. use关键字

使用`use`关键字将路径导入到作用域内

- 导入仍然遵循私有性规则

## 1.1 使用use引用绝对路径

例:

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// 这里使用use的效果类似文件系统中的软链接
// 引入之后 hosting就可以在当前作用域(crate root)内直接使用了
// 相当于模块hosting是在crate root下定义的
use crate::front_of_house::hosting;

// 其实等价于如下代码:

// mod front_of_house { pub fn add_to_waitlist() {} }

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

## 1.2 使用use引用相对路径

```rust
mod front_of_house {
    pub mod hosting {
        // 此处处于hosting模块中
        // hosting的父模块是front_of_house
        // front_of_house的父模块是crate root
        use super::super::eat_at_restaurant;

        pub fn add_to_waitlist() {}

        fn use_super() {
            eat_at_restaurant();
        }
    }
}

// 此处处于root crate 所以想要调用root crate的子模块中的内容
// 直接指定路径即可
use front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

## 1.3 use的习惯用法

- 针对模块中的函数,通常引入这个函数所属的模块即可,很少有直接将这个函数引入的情况.因为这样没法通过函数调用,看出这个函数到底是在本模块中定义的,还是在其他模块中定义的
- 针对struct,enum或其他语法项:指定完整路径(指定到本身)

例:

```rust
// 路径直接到struct本身 而非struct所在的模块
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
```

- 同名语法项:指定到父级
  - 在2个不同的模块中,有同名的struct,enum,trait等语法项,此时使用`use`引入到父级即可

例:标准库中有`std::fmt`和`std::io`两个模块,这两个模块中都有`Result`这个枚举,此时需要指定到父级再引入

```rust
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
```

# PART2. 同名条目的另一种解决方案:`as`关键字

- 使用`as`关键字,可以将引入的模块指定一个本地的别名

```rust
// 由于有同名的语法项,因此在引入时,引入到enum,然后给同名的语法项起别名
use std::fmt::Result as FmtResult;
use std::io::Result as IoResult;

fn main() {
    println!("Hello, world!");
}

fn f1() -> FmtResult {
    Ok(())
}

fn f2() -> IoResult<()> {
    Ok(())
}
```