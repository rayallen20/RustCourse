# PART1. 13.1节中的运动计划问题

现代码如下:

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

// 本函数用于模拟生成健身计划的过程
fn generate_workout(intensity: u32, random_number: u32) {

    // 此处如果只定义闭包而不调用这个闭包 编译器会报错 因为无法推断num的类型
    // 本闭包用于模拟一个耗时的计算过程
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure(intensity));
        }
    }
}
```

问题之前说过,在`intensity`小于25时,闭包被调用了2次

解决方案:创建一个struct,该struct持有闭包和闭包的调用结果,这样做的好处在于:

- 只会在需要结果时才执行该闭包
- 闭包执行之后的结果可以缓存

这个模式通常叫做记忆化(memorization)或延迟计算(lazy evaluation)

# PART2. 如何让struct持有闭包

- struct的定义需要知道所有字段的类型
  - 因此需要指明闭包的类型
- 每个闭包实例都有自己唯一的匿名类型,即使2个闭包的签名完全一样,他们的类型也是不同的
- 所以需要使用泛型和Trait Bounds来定义struct

# PART3. Fn Trait

Fn Trait由标准库提供

所有的闭包都至少实现了以下trait之一:

- Fn
- FnMut
- FnOnce

这3个的区别以后再讲.

# PART4. 使用struct持有闭包和闭包的调用结果

```rust
use std::thread;
use std::time::Duration;

// T表示闭包的类型
struct Cacher<T>
where T: Fn(u32) -> u32
{
  calculation: T,
  // 此处使用Option枚举来存储计算结果 未计算时为None 计算后为Some(u32)
  value: Option<u32>,
}

impl <T> Cacher<T>
where T: Fn(u32) -> u32
{
  fn new(calculation: T) -> Cacher<T> {
    Cacher {
      calculation,
      value: None,
    }
  }

  fn value(&mut self, arg: u32) -> u32 {
    match self.value {
      // 如果已经计算过 则直接返回保存的结果
      Some(v) => v,
      None => {
        // 如果未计算过 则调用闭包计算并保存结果
        let v = (self.calculation)(arg);
        self.value = Some(v);
        v
      }
    }
  }
}

fn main() {
  let simulated_user_specified_value = 10;
  let simulated_random_number = 7;

  generate_workout(simulated_user_specified_value, simulated_random_number);
}

// 本函数用于模拟生成健身计划的过程
fn generate_workout(intensity: u32, random_number: u32) {

  // 本闭包用于模拟一个耗时的计算过程
  let expensive_closure = |num| {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
  };

  let mut cacher :Cacher<fn(u32) -> u32> = Cacher::new(expensive_closure);

  if intensity < 25 {
    println!("Today, do {} pushups!", cacher.value(intensity));
    println!("Next, do {} situps!", cacher.value(intensity));
  } else {
    if random_number == 3 {
      println!("Take a break today! Remember to stay hydrated!");
    } else {
      println!("Today, run for {} minutes!", cacher.value(intensity));
    }
  }
}
```

# PART5. 使用缓存器(Cacher)实现的限制

1. 同一个Cacher实例针对不同的arg,最终返回的value是同样的值,都是第一次计算的结果
2. 只能接收u32类型的参数

```rust
use std::thread;
use std::time::Duration;

// T表示闭包的类型
struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    // 此处使用Option枚举来存储计算结果 未计算时为None 计算后为Some(u32)
    value: Option<u32>,
}

impl <T> Cacher<T>
where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            // 如果已经计算过 则直接返回保存的结果
            Some(v) => v,
            None => {
                // 如果未计算过 则调用闭包计算并保存结果
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

// 本函数用于模拟生成健身计划的过程
fn generate_workout(intensity: u32, random_number: u32) {

    // 本闭包用于模拟一个耗时的计算过程
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    let mut cacher :Cacher<fn(u32) -> u32> = Cacher::new(expensive_closure);

    if intensity < 25 {
        println!("Today, do {} pushups!", cacher.value(intensity));
        println!("Next, do {} situps!", cacher.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", cacher.value(intensity));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);
        let v1 = c.value(1);
        let v2 = c.value(2);
        println!("v1: {}, v2: {}", v1, v2);
        assert_eq!(v2, 2)
    }
}
```

```
cargo test
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.04s
     Running unittests src/main.rs (target/debug/deps/closure_example-4173c4ee3d20459f)

running 1 test
test tests::call_with_different_values ... FAILED

failures:

---- tests::call_with_different_values stdout ----
v1: 1, v2: 1
thread 'tests::call_with_different_values' panicked at src/main.rs:78:9:
assertion `left == right` failed
  left: 1
 right: 2
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::call_with_different_values

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--bin closure_example`
```

先来解决第2个问题:让闭包能够接收不同类型的参数

```rust
use std::fmt::Display;
use std::thread;
use std::time::Duration;

// T表示闭包的类型
struct Cacher<T, E>
    where T: Fn(E) -> E,
          E: Display + Copy
{
    calculation: T,
    // 此处使用Option枚举来存储计算结果 未计算时为None 计算后为Some(u32)
    value: Option<E>,
}

impl <T, E> Cacher<T, E>
where T: Fn(E) -> E,
    E: Display + Copy
{
    fn new(calculation: T) -> Cacher<T, E> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: E) -> E {
        match self.value {
            // 如果已经计算过 则直接返回保存的结果
            Some(v) => v,
            None => {
                // 如果未计算过 则调用闭包计算并保存结果
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

// 本函数用于模拟生成健身计划的过程
fn generate_workout(intensity: u32, random_number: u32) {

    // 本闭包用于模拟一个耗时的计算过程
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    let mut cacher :Cacher<fn(u32) -> u32, u32> = Cacher::new(expensive_closure);

    if intensity < 25 {
        println!("Today, do {} pushups!", cacher.value(intensity));
        println!("Next, do {} situps!", cacher.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", cacher.value(intensity));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);
        let v1 = c.value(1);
        let v2 = c.value(2);
        println!("v1: {}, v2: {}", v1, v2);
        assert_eq!(v2, 2)
    }

    #[test]
    fn call_with_genericity_values() {
        let mut c = Cacher::new(|a :u32| -> u32 {a});
        let v1 = c.value(1);
        assert_eq!(v1, 1);

        let mut c2 = Cacher::new(|a :f32| -> f32 {a});
        let v2 = c2.value(2.7);
        assert_eq!(v2, 2.7);
    }
}
```

```
cargo test call_with_genericity_values
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running unittests src/main.rs (target/debug/deps/closure_example-4173c4ee3d20459f)

running 1 test
test tests::call_with_genericity_values ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s
```

再来解决第1个问题:让同一个Cacher实例针对不同的arg,最终返回的value是不同的值

```rust
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

// T表示闭包的类型
struct Cacher<T, E>
where T: Fn(E) -> E,
      E: Display + Copy + Eq + Hash
{
  calculation: T,
  result_collection: HashMap<E, E>,
}

impl <T, E> Cacher<T, E>
where T: Fn(E) -> E,
      E: Display + Copy + Eq + Hash
{
  fn new(calculation: T) -> Cacher<T, E> {
    Cacher {
      calculation,
      result_collection: HashMap::new(),
    }
  }

  fn value(&mut self, arg: E) -> E {
    if self.result_collection.contains_key(&arg) {
      self.result_collection.get(&arg).unwrap().clone()
    } else {
      let v = (self.calculation)(arg);
      self.result_collection.insert(arg, v);
      v
    }
  }
}

fn main() {
  let simulated_user_specified_value = 10;
  let simulated_random_number = 7;

  generate_workout(simulated_user_specified_value, simulated_random_number);
}

// 本函数用于模拟生成健身计划的过程
fn generate_workout(intensity: u32, random_number: u32) {

  // 本闭包用于模拟一个耗时的计算过程
  let expensive_closure = |num| {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
  };

  let mut cacher :Cacher<fn(u32) -> u32, u32> = Cacher::new(expensive_closure);

  if intensity < 25 {
    println!("Today, do {} pushups!", cacher.value(intensity));
    println!("Next, do {} situps!", cacher.value(intensity));
  } else {
    if random_number == 3 {
      println!("Take a break today! Remember to stay hydrated!");
    } else {
      println!("Today, run for {} minutes!", cacher.value(intensity));
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);
    let v1 = c.value(1);
    assert_eq!(v1, 1);

    let v2 = c.value(2);
    assert_eq!(v2, 2)
  }

  #[test]
  fn call_with_genericity_values() {
    let mut c = Cacher::new(|a :u32| -> u32 {a});
    let v1 = c.value(1);
    assert_eq!(v1, 1);

    let mut c2 = Cacher::new(|a :i64| -> i64 {a});
    let v2 = c2.value(2);
    assert_eq!(v2, 2);
  }
}
```

```
cargo test
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/main.rs (target/debug/deps/closure_example-4173c4ee3d20459f)

running 2 tests
test tests::call_with_different_values ... ok
test tests::call_with_genericity_values ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

注:

- 此处由于使用arg作为HashMap的key,所以arg需要实现Copy, Eq, Hash Trait
- 而f32类型并没有实现Copy Trait,所以无法作为HashMap的key
- 因此修改测试用例`call_with_genericity_values`中的`c2`的类型为`i64`