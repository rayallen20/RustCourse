# PART1. Send Trait和Sync Trait

- Rust语言的并发特性较少,目前讲的并发特性都是来自标准库(而不是语言本身)
- 无需局限于标准库的并发,可以自己实现并发
- 但在Rust语言中有2个并发概念:
  - `sync::marker::Send` Trait和`sync::marker::Sync` Trait
  - 这2个Trait都是标签Trait,不包含任何方法,只是用来标记类型是否满足某种并发特性

```rust
#[stable(feature = "rust1", since = "1.0.0")]
#[cfg_attr(not(test), rustc_diagnostic_item = "Send")]
#[diagnostic::on_unimplemented(
    message = "`{Self}` cannot be sent between threads safely",
    label = "`{Self}` cannot be sent between threads safely"
)]
pub unsafe auto trait Send {
    // empty.
}
```

```rust
pub unsafe auto trait Sync {
    // FIXME(estebank): once support to add notes in `rustc_on_unimplemented`
    // lands in beta, and it has been extended to check whether a closure is
    // anywhere in the requirement chain, extend it as such (#48534):
    // ```
    // on(
    //     closure,
    //     note="`{Self}` cannot be shared safely, consider marking the closure `move`"
    // ),
    // ```

    // Empty
}
```

# PART2. Send Trait

该Trait允许线程间转移所有权,实现该Trait的类型可以在线程间转移所有权

Rust中几乎所有类型都实现了Send Trait,除了:

- `Rc<T>`类型没有实现Send Trait,因为Rc<T>类型是引用计数类型,不是线程安全的
- 任何完全由实现了Send Trait的类型组成的类型,也会自动实现Send Trait
- 除了原始指针外,几乎所有类型都实现了Send Trait

# PART3. Sync Trait

该Trait允许多线程同时访问数据,实现该Trait的类型可以在线程间共享数据

- 实现了Sync Trait的类型可以安全的被多个线程引用
- 也就是说: 如果`T`实现了Sync Trait,那么`&T`就实现了Sync Trait
  - 引用可以被安全地送往另一个线程
- 基础类型都实现了Sync Trait
- 任何完全由实现了Sync Trait的类型组成的类型,也会自动实现Sync Trait
  - 但是,`Rc<T>`类型没有实现Sync Trait,因为Rc<T>类型是引用计数类型,不是线程安全的
  - `RefCell<T>`和`Cell`家族也都不是Sync的,因为它们是用来实现内部可变性的,不是线程安全的
  - `Mutex<T>`和`RwLock<T>`类型是Sync的,因为它们是用来实现线程安全的

# PART4. 手动实现Send Trait和Sync Trait是不安全的

记住这句话就可以了