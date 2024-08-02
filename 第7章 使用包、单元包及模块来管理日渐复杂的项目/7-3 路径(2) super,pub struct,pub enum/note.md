# PART1. super

用于访问父级模块路径中的内容,类似文件系统中的..

例:

```rust
fn server_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // server_order()位于根模块(crate root)中
        super::server_order();

        // 使用绝对路径的等价形式
        crate::server_order();
    }

    fn cook_order() {}
}
```

注意,这里不需要给`server_order`函数添加`pub`修饰符.因为模块`back_of_house`是根模块的子模块,所以可以访问根模块中的私有函数.

# PART2. pub struct

`pub`放在`struct`前:

- struct是公共的
- struct的字段是私有的
- 字段前加`pub`修饰符,可以使字段成为公有的

例:

```rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        // 关联函数 用于提供Breakfast实例
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    meal.seasonal_fruit = String::from("blueberries"); // error: field `seasonal_fruit` of struct `back_of_house::Breakfast` is private
}
```

# PART3. pub enum

`pub`放在`enum`前:

- enum是公共的
- enum的变体也都是公共的
  - 这一点违反了rust的规则,rust在默认情况下,基本上所有的语法项都是私有的
  - 这样设计是因为,enum的变体如果是私有的,那么就无法在外部使用这个enum的全部变体,这样就违背了enum的设计初衷

例:

```rust
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}
```