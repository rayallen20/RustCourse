# PART1. 内部可变性(interior mutability)

- 内部可变性是Rust的设计模式之一
- 它允许你在只持有不可变引用的前提下对数据进行修改
  - 类似的行为通常会被借用规则禁止
  - 为了在只持有不可变引用的前提下修改数据,内部可变性模式在它的数据结构中使用了`unsafe`代码来绕过Rust正常的可变性和借用规则

# PART2. `RefCell<T>`

- 与`Rc<T>`不同,`RefCell<T>`类型代表了其持有数据的唯一所有权
- `RefCell<T>`只能用于单线程的场景

先回忆一下借用规则:

1. 在任何给定的时间里:

   - 要么只能拥有1个可变引用
   - 要么只能拥有多个任意数量的不可变引用

2. 引用总是有效的

# PART3. `RefCell<T>`与`Box<T>`的区别

|        `Box<T>`        |    `RefCell<T>`     |
|:----------------------:|:-------------------:|
|   **编译阶段**强制代码遵守借用规则   |  只会在**运行时**检查借用规则   |
| 不满足借用规则会**出现错误(编译错误)** | 不满足借用规则会**触发panic** |

# PART4. 借用规则在不同阶段进行检查的比较

|    编译阶段     |              运行时               |
|:-----------:|:------------------------------:|
|   尽早暴露问题    |  问题的暴露延后(延后到运行时),甚至可能延后到生产环境   |
|  没有任何运行时开销  | 因为在运行时进行借用规则检查(借用计数)而产生些许的性能损失 |
| 对大多数场景是最佳选择 | 实现某些特定的内存安全场景(比如在不可变环境中修改自身数据) |
| 是Rust的默认行为  |                                |

对"实现某些特定的内存安全场景"的解释:

Rust编译器在编译阶段就检查所有代码,其中大部分的代码都能够在编译阶段就能够分析清楚.

对于能够分析清楚的这部分代码,如果没有问题就通过编译,如果有问题那么在编译阶段也就暴露问题了.

而且Rust编译器本质上是非常保守的,因为某些代码是无法在编译阶段分析清楚的,也就是说这部分代码在编译阶段无法完成分析.

而Rust编译器就会拒绝掉所有不符合所有权规则的代码,哪怕这些代码没有任何问题,也会被拒绝.这就是Rust编译器的保守性.

之所以Rust编译器设计的这么保守,是因为Rust编译器一旦放行了某个可能有问题的程序,那么Rust对安全性的保证就将直接破产.

虽然拒绝掉某些正确的程序可能会对开发者带来不便,但至少不会产生灾难性的后果.

但是针对这些无法在编译阶段分析清楚的代码,如果开发者可以保证它们是满足借用规则的,那么就可以使用`RefCell<T>`来绕过Rust编译器的检查.

# PART5. 选择`Box<T>`、`Rc<T>`、`RefCell<T>`的依据

|          |     `Box<T>`      |    `Rc<T>`     |   `RefCell<T>`    |
|:--------:|:-----------------:|:--------------:|:-----------------:|
| 同一数据的所有者 |        1个         |       多个       |        一个         |
| 可变性、借用检查 | 允许可变、不可变借用(编译时检查) | 允许不可变借用(编译时检查) | 允许可变、不可变借用(运行时检查) |

- 其中,即使`RefCell<T>`本身不可变,但仍然可以修改其中存储的值

`Box<T>`可变借用的示例:

```rust
fn main() {
    let mut x = Box::new(5);
    println!("x = {}", x);

    let mutable_value = &mut *x;
    *mutable_value += 1;
    println!("x = {}", x);
}
```

```
cargo run
   Compiling mutable_box v0.1.0 (/mutable_box)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.98s
     Running `target/debug/mutable_box`
x = 5
x = 6
```

# PART6. 内部可变性:可变地借用一个不可变的值

借用规则中有这样一条推论:无法可变地借用一个不可变的值

```rust
fn main() {
    let x = 5;
    let y = &mut x;  // error: cannot borrow immutable local variable `x` as mutable
}
```

但是在某些特定情况下,我们需要这样的一种值:

- 对外部代码表现为不可变
- 能够在方法内部修改自身的值
- 除了这个值的成员方法外,其余的代码都无法修改这个值

使用`RefCell<T>`可以获得这种内部可变性.但是,`RefCell<T>`并不是完全绕开了借用规则,虽然使用内部可变性通过了编译阶段的借用检查,

但借用检查的工作仅仅是延后到了运行阶段而已.如果你违反了借用规则,那么会得到一个panic,而非是一个编译时的错误.

## 6.1 测试替代(test double)与模拟对象(mock object)

- 测试替代(test double): 是一个通用的编程概念,它代表了那些在测试工作中被用作其他类型替代品的对象
- 模拟对象(mock object): 指代了测试替代中的某些特定类型,它们会承担起记录测试过程的工作.我们可以利用这些记录来断言测试工作的运行是否正确

Rust没有和其他语言中类似的对象概念,也同样没有在标准库中提供模拟对象的测试功能.但是,我们可以自行定义一个结构体来实现与模拟对象相同的功能

## 6.2 问题的产生

设计的测试场景如下:

我们希望开发一个记录并对比当前值与最大值的库,它会基于当前值与最大值之间的接近程度向外传递信息.

例如:这个库可以记录用户调用不同API的次数,并将它们与设置的调用限额进行比较

我们只会在这个库中记录当前值与最大值的接近程度,以及决定何时显示何种信息

```rust
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where T: Messenger {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        }
    }
}
```

- 这段代码中,`Messenger` Trait定义了一个`send()`方法,该方法接收一个`self`的不可变应用和一段文本内容作为参数(这里我们不需要考虑它如何将文本发送出去)
  - 在后续测试替代的过程中,我们创建的模拟对象就需要实现这个trait
- `LimitTracker`需要使用一个`Messenger` Trait和一个`max`值来完成初始化
- `LimitTracker`的`set_value()`方法会根据当前值(`value`)与最大值(`max`)之间的接近程度作出不同的行为
  - 但是,`set_value()`方法并不会产生任何的返回值,因此我们无法为该方法进行断言测试
  - 因此,我们需要在后续的测试过程中,创建模拟对象来记录`set_value()`方法的调用情况

```rust
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where T: Messenger {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessenger {
        // 记录发送的消息
        sent_messages: Vec<String>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger { sent_messages: vec![] }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // 此处接口定义的 self 是一个不可变引用
            // 但是我们又需要修改 self.sent_messages
            // 如果改成 &mut self,则MockMessenger就不再是Messenger的实现
            self.sent_messages.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.len(), 1);
    }
}
```

```
 cargo test
   Compiling interior_mutability_example_1 v0.1.0 (/interior_mutability_example_1)
error[E0596]: cannot borrow `self.sent_messages` as mutable, as it is behind a `&` reference
  --> src/lib.rs:56:13
   |
56 |             self.sent_messages.push(String::from(message));
   |             ^^^^^^^^^^^^^^^^^^ `self` is a `&` reference, so the data it refers to cannot be borrowed as mutable
   |
help: consider changing this to be a mutable reference
   |
2  |     fn send(&mut self, msg: &str);
   |             ~~~~~~~~~

For more information about this error, try `rustc --explain E0596`.
error: could not compile `interior_mutability_example_1` (lib test) due to 1 previous error
warning: build failed, waiting for other jobs to finish...
```

也就是说我们现在处于一种两难的情况:

- 把`MockMessenger`的`send()`方法的参数改成`&mut self`就无法实现`Messenger` Trait
- 保持`MockMessenger`的`send()`方法的参数为`&self`就无法修改`MockMessenger.sent_messages`

## 6.3 使用`RefCell<T>`在测试中模拟对象

这种场景下就可以使用`RefCell<T>`来保证`MockMessenger`的`send()`方法的参数为`&self`,同时又能修改`MockMessenger.sent_messages`(内部可变性)

```rust
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where T: Messenger {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use super::*;

    struct MockMessenger {
        // 使用RefCell<T>类型使得该字段可以在不可变的self上进行可变的操作
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            // send_messages字段的类型为RefCell<Vec<String>> 而不是Vec<String>
            MockMessenger { sent_messages: RefCell::new(vec![]) }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // RefCell<T>.borrow_mut()返回RefMut<T>类型,该类型它实现了DerefMut trait,所以可以直接使用*操作符
            // RefMut<T>智能指针可以修改其内部的值
            // 但是RefCell<T>类型同样会进行借用规则检查,只不过这个检查的过程延后到运行时了:
            // 每次调用RefCell<T>.borrow_mut()方法时,RefCell<T>会检查是否有其他代码正在使用RefCell<T>的不可变引用
            // 如果有,RefCell<T>.borrow_mut()方法会panic!
            // 可以有多个不可变引用,但是只能有一个可变引用 (这个规则是符合Rust的借用规则的)
            // 因此,调用RefCell<T>.borrow_mut()方法时,RefCell<T>同样会检查是否有其他代码正在使用RefCell<T>的不可变引用
            // 如果有,RefCell<T>.borrow_mut()方法也会panic!
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        // RefCell<T>.borrow()方法返回Ref<T>类型,该类型实现了Deref trait,所以可以直接使用*操作符
        // Ref<T>也是一种智能指针,通过该指针可以访问RefCell<T>内部的值
        // 但是,Ref<T>类型只能用于访问内部值,不能用于修改内部值
        // 调用RefCell<T>.borrow()方法时同样会进行借用规则检查,
        // 若调用该方法时,存在RefCell<T>的可变引用,则会panic!
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);

        // 总结:
        // RefCell<T>在运行时强制执行以下借用规则:
        // 可以有多个不可变引用(通过RefCell<T>.borrow()获取)
        // 或者有一个可变引用(通过RefCell<T>.borrow_mut()获取)
        // 当存在可变借用时,不允许存在任何不可变借用
        // 当存在不可变借用时,不允许存在任何可变借用
    }
}
```

```
cargo test
   Compiling interior_mutability_example_2 v0.1.0 (/interior_mutability_example_2)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 5.35s
     Running unittests src/lib.rs (target/debug/deps/interior_mutability_example_2-a2ece08ec283c389)

running 1 test
test tests::it_sends_an_over_75_percent_warning_message ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests interior_mutability_example_2

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

# PART7. 使用`RefCell<T>`在运行时记录借用信息

2个方法(安全接口):

- `borrow()`方法:
  - 返回智能指针`Ref<T>`,该类型实现了Deref Trait,可以通过`*`操作符访问内部值
  - 可以简单理解为`RefCell<T>.borrow()`返回了一个`&T`类型
- `borrow_mut()`方法:
  - 返回智能指针`RefMut<T>`,该类型实现了DerefMut Trait,可以通过`*`操作符访问内部值
  - 可以简单理解为`RefCell<T>.borrow_mut()`返回了一个`&mut T`类型
- `RefCell<T>`会记录当前存在多少个活跃的`Ref<T>`和`RefMut<T>`智能指针
  - 每次调用`RefCell<T>.borrow()`: 其不可变借用的计数就加1
  - 任何一个`Ref<T>`的值离开作用域被释放时: 其不可变借用的计数就减1
  - 每次调用`RefCell<T>.borrow_mut()`: 其可变借用的计数就加1
  - 任何一个`RefMut<T>`的值离开作用域被释放时: 其可变借用的计数就减1

`RefCell<T>`的运行时借用规则:

- 在任何一个给定时间里,只允许有多个不可变借用或者一个可变借用

# PART8. 将`Rc<T>`和`RefCell<T>`结合使用来实现一个拥有多重所有权的可变数据

- `Rc<T>`: 允许多个所有者持有同一数据,但只能提供针对数据的不可变访问
- 但是如果在`Rc<T>`内部存储`RefCell<T>`,即`Rc<RefCell<T>>`,那么就可以定义出拥有多个所有者且能够进行修改的值了

```rust
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(List::Cons(Rc::clone(&value), Rc::new(List::Nil)));

    let b = List::Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));

    let c = List::Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    // 这行代码实际上等价于: *((*value).borrow_mut()) += 10;
    // step1. 自动解引用
    // 由于 value 的类型为 Rc<T>,该类型实现了 Deref trait,因此Rust会自动解引用以便调用borrow_mut()方法
    // step2. 调用 borrow_mut() 方法
    // 该方法返回了一个 RefMut<T> 类型的智能指针,该指针实现了 DerefMut trait,因此可以通过 * 运算符来解引用
    // step3. 解引用
    // 智能指针RefMut<T>也实现了 DerefMut trait,因此可以通过 * 运算符来解引用,从而得到一个可变的内部值(这里是一个i32类型的值)
    *value.borrow_mut() += 10;

    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);
}
```

```
cargo run
   Compiling use_rc_and_ref_cell v0.1.0 (/use_rc_and_ref_cell)
...
warning: `use_rc_and_ref_cell` (bin "use_rc_and_ref_cell") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.94s
     Running `target/debug/use_rc_and_ref_cell`
Cons(RefCell { value: 15 }, Nil)
Cons(RefCell { value: 3 }, Cons(RefCell { value: 15 }, Nil))
Cons(RefCell { value: 4 }, Cons(RefCell { value: 15 }, Nil))
```

# PART9. 其他可实现内部可变性的类型

- `Cell<T>`: 通过复制来访问数据(`RefCell<T>`是通过借用来访问数据的)
- `Mutex<T>`: 用于实现跨线程场景下的内部可变性模式