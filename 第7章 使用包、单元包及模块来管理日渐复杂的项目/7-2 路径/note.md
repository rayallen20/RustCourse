# PART1. 路径

Rust中的路径和文件系统中的路径差不多

路径的2种形式:

- 绝对路径:从crate root开始,使用crate名称或字面量`crate`作为开始
- 相对路径:从当前模块开始,使用`self`或`super`标识符作为开始

标识符之间使用`::`分隔

例:

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // 绝对路径(使用crate关键字)
    // 函数eat_at_restaurant()和模块front_of_house都在lib.rs中
    // lib.rs是library crate的根文件 所以函数eat_at_restaurant()和模块front_of_house隐式地组成了根模块(crate root)
    // 所以这里的crate关键字 指的就是crate root 也就是函数eat_at_restaurant()和模块front_of_house的位置
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    // 函数eat_at_restaurant()和模块front_of_house在crate中处于同级
    // 因此直接从模块名(front_of_house)开始就可以了
    front_of_house::hosting::add_to_waitlist();
}

```

至于究竟使用绝对路径还是相对路径,取决于你定义语法项的代码和你调用语法项的代码之间的关系,二者要是在同一个模块中,就可以使用相对路径,否则就要使用绝对路径

编译:

```bash
cargo build
   Compiling restaurant v0.1.0 (/restaurant)
error[E0603]: module `front_of_house` is private
  --> src/lib.rs:19:28
   |
19 |     crate::front_of_house::front_of_house::add_to_waitlist();
   |                            ^^^^^^^  --------------- function `add_to_waitlist` is not publicly re-exported
   |                            |
   |                            private module
   |
note: the module `front_of_house` is defined here
  --> src/lib.rs:2:5
   |
2  |     mod front_of_house {
   |     ^^^^^^^^^^^

error[E0603]: module `front_of_house` is private
  --> src/lib.rs:24:21
   |
24 |     front_of_house::front_of_house::add_to_waitlist();
   |                     ^^^^^^^  --------------- function `add_to_waitlist` is not publicly re-exported
   |                     |
   |                     private module
   |
note: the module `front_of_house` is defined here
  --> src/lib.rs:2:5
   |
2  |     mod front_of_house {
   |     ^^^^^^^^^^^

For more information about this error, try `rustc --explain E0603`.
error: could not compile `restaurant` (lib) due to 2 previous errors
```

module `hosting` is private:模块`hosting`是私有的

# PART2. 私有边界(privacy boundary)

- 模块不仅可以组织代码,还可以定义私有边界
- 如果想把函数或struct设置为私有,可以将它放置在某个模块中
- rust中所有的语法项(函数、方法、struct、enum、模块和常量)默认都是私有的
- 父级模块无法访问子模块中的私有条目
- 子模块中可以使用所有父级模块中的条目,包括私有条目

## 2.1 pub关键字

在刚刚的例子中:

- 模块`front_of_house`是私有的
- 模块`hosting`是私有的
- 函数`add_to_waitlist`是私有的

因此,应该将模块和函数都设置为公有:

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();

    front_of_house::hosting::add_to_waitlist();
}
```

注意,模块`front_of_house`不需要加`pub`关键字,因为函数`eat_at_restaurant()`和模块`front_of_house`在同一个模块(隐式声明的根模块crate)中,所以可以直接访问
