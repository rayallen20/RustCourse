# PART1. Rust是面向对象编程语言吗?

- Rust受到多种编程范式的影响,包括面向对象
- 面向对象通常包含以下特性: 命名对象、继承、封装

# PART2. 对象包含数据和行为

- "设计模式四人帮"在《设计模式》一书中给面向对象的定义:
  - 面向对象的程序由对象组成
  - 对象包装了数据和操作这些数据的过程,这些过程通常被称为方法或操作
- 基于此定义: Rust是面向对象的
  - struct、enum包含数据
  - impl块为之提供方法
  - 但带有方法的struct、enum并没有被称为对象

# PART3. 封装

- 封装: 对象外部的、调用该对象的代码无法直接访问对象内部的实现细节,唯一可以与对象进行交互的方法就是通过它公开的API
- Rust中: pub关键字可以决定那些模块、结构体、枚举、函数、方法是公开的

例: 定义一个名为`AveragedCollection`的结构体,该结构体中包含一个存储`i32`元素的Vec.

该结构体中还包含一个用于存储Vec中元素的平均值的字段,其目的是避免在每次读取元素平均值时重复计算.

```rust
/// 本结构体用于维护(添加/删除)一个i32类型的集合,并在集合每次发生变化时更新平均值
pub struct AveragedCollection {
    /// 本字段用于存储所有的数字
    list: Vec<i32>,
    /// 本字段用于存储所有数字的平均值 避免每次读取平均值时都重新计算
    average: f64,
}
```

- 该结构体为`pub`,但是其内部的字段都是私有的

然后,我们为其添加`add()`、`remove()`、`average()`方法,用于添加、删除元素,以及获取当前元素的平均值.

```rust
/// 本结构体用于维护(添加/删除)一个i32类型的集合,并在集合每次发生变化时更新平均值
pub struct AveragedCollection {
    /// 本字段用于存储所有的数字
    list: Vec<i32>,
    /// 本字段用于存储所有数字的平均值 避免每次读取平均值时都重新计算
    average: f64,
}

impl AveragedCollection {
    /// 本方法用于根据结构体实例中当前的数字集合计算平均值
    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }

    /// 本方法用于向集合中添加一个数字,并更新平均值
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    /// 本方法用于删除集合末尾的数字,并更新平均值
    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            },
            None => None
        }
    }

    /// 本方法用于获取当前集合的平均值
    pub fn average(&self) -> f64 {
        self.average
    }
}
```

- `add()`方法、`remove()`方法、`average()`方法都是`pub`的,可以被外部调用
  - 这些方法要么修改了结构体实例的内部状态,要么读取了结构体实例的内部状态
- `update_average()`方法是私有的,只能被结构体实例内部调用
  - 该方法用于更新结构体实例中的`average`字段
  - 而这个算法过程是不需要被外部调用者知道的

# PART4. 继承

- 继承: 使对象可以沿用另外一个对象的数据和行为,且无需重复定义相关代码
- Rust中没有继承的概念
- 使用继承的原因:
  - 代码复用
    - Rust中可以使用trait的默认方法来进行代码复用
  - 多态
    - Rust中使用泛型和trait约束来实现多态(限定参数化多态 bounded parametric)
    - 需要注意的是,泛型和trait约束在使用时,是无法动态绑定的,因此无法实现动态多态
      - 意思是: 假设有一个`Animal` trait,有一个`Dog`结构体和一个`Cat`结构体,这两个结构体都实现了`Animal` trait
      - 那么在使用时,假设此时有一个函数,其签名为`fn make_animal_talk<T: Animal>(animals: Vec<T>)`
      - 那么在调用`make_animal_talk()`时,你无法得到一个既有`Dog`又有`Cat`的`Vec`参数
      - 因为Rust是静态类型语言,无法在运行时动态绑定

```rust
trait Animal {
    fn name(&self) -> String;
    fn talk(&self) {
        println!("{} cannot talk", self.name());
    }
}

struct Dog {
    name: String,
}

impl Animal for Dog {
    fn name(&self) -> String {
        self.name.clone()
    }
}

struct Cat {
    name: String,
}

impl Animal for Cat {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn talk(&self) {
        println!("{} says meow", self.name());
    }
}

fn make_animal_talk<T: Animal>(animals: Vec<T>) {
    for animal in animals {
        animal.talk();
    }
}

fn main() {
    let dog = Dog {
        name: String::from("Dog"),
    };
    let cat = Cat {
        name: String::from("Cat"),
    };

    let animals: Vec<Animal> = vec![dog, cat];  // error: doesn't have a size known at compile-time
    make_animal_talk(animals);
}
```

- 很多语言都不使用继承作为内置的程序设计方案了