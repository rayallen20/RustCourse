# PART1. 将值传递给函数

在语义上,将值传递给函数和把值赋给变量是一样的:将值传递给函数将发生移动或复制

```rust
fn main() {
    let s = String::from("hello");
    take_ownership(s); // s是分配在Heap上的,传入函数时所有权发生了移动(move),此后s不能再使用
    // println!("{}", s); // 编译错误: value borrowed here after move

    let x = 5;
    makes_copy(x); // x是i32类型,实现了Copy trait,因此传入函数时传递的是x的副本(传入时发生了copy),而不是x本身,所以x仍然有效
    println!("{}", x);
} // s和x都离开作用域 s由于已经失效,因此不会调用drop函数,而x由于实现了Copy trait,因此没有drop函数

fn take_ownership(some_string: String) {
    println!("{}", some_string);
} // some_string离开作用域 调用drop函数释放内存

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // some_integer离开作用域,由于实现了Copy trait而不能实现Drop trait,因此不会调用drop函数
```

# PART2. 返回值与作用域

函数的返回值在返回时同样也会发生所有权的转移

```rust
fn main() {
    // some_string的值(也就是数据hello)的所有权被移动到s1
    let s1 = gives_ownership();

    let s2 = String::from("hello");

    // step1. s2的值(也就是数据hello)的所有权被移动到函数内部
    // step3. s3获得数据hello的所有权
    let s3 = takes_and_gives_back(s2);

    // println!("{}", s2) // error: value borrowed here after move
} // step4. s1和s3离开作用域,调用drop函数,释放内存; s2的值的所有权已经被移动,所以不会发生任何事情

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
} // some_string作为返回值,所有权移动到调用者,本例中是s1

fn takes_and_gives_back(a_string: String) -> String {
    a_string
} // step2. takes_and_gives_back函数获得hello的所有权,并将所有权返回给调用者,本例中是s3
```

一个变量的所有权总是遵循同样的模式:

- 把一个值赋给其他变量时,会发生移动
- 当一个包含Heap数据的变量离开作用域时,它的值就会被drop函数清理,除非数据的所有权移动到另一个变量上了

# PART3. 如何让函数使用某个值,但又不获得其所有权?

```rust
fn main() {
    let s1 = String::from("hello");

    // step2. s2 接收返回的所有权
    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);

    // 实际上s1还是失去了所有权 因此这个例子中函数calculate_length还是获得了s1的数据(hello)的所有权
    // 只是最终通过返回值的方式又将这个数据的所有权还给了调用它的函数
    // println!("{}", s1); // error[E0382]: borrow of moved value: `s1`
}

fn calculate_length(s: String) -> (String, usize) {
    // 这里length是该函数本该返回的计算结果
    let length = s.len();
    // 但是为了保证调用者仍旧保持对s的数据的所有权,因此不得不将s返回
    (s, length)
} // step1. 将 s 的所有权返回给调用它的函数
```

也就是说,如果调用者想保留对数据的所有权,则被调用的函数不得不将数据返回,这样就会导致数据的所有权在函数之间来回传递,这样的操作显然是不够优雅的