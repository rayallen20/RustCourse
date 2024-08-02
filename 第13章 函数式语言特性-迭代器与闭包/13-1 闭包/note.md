# PART1. 什么是闭包

闭包: 可以捕获其运行环境的匿名函数

闭包的特点:

- 是匿名函数
- 可以保存为变量、作为参数传递、作为返回值返回
- 可以在一个地方创建闭包,然后在另一个上下文中调用闭包来完成运算
- 可以从它定义的作用域中捕获变量

# PART2. 例-生成自定义运动计划程序

- 算法的逻辑不是重点,重点是算法中的计算过程需要几秒钟的时间
- 目标:不让用户发生不必要的等待
    - 仅在必要时调用该算法
    - 只调用1次

代码如下:

```rust
use std::thread;
use std::time::Duration;

fn main() {
  let simulated_user_specified_value = 10;
  let simulated_random_number = 7;

  generate_workout(simulated_user_specified_value, simulated_random_number);
}

// 本函数用于模拟一个耗时的计算过程
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

// 本函数用于模拟生成健身计划的过程
fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!("Today, do {} pushups!", simulated_expensive_calculation(intensity));
        println!("Next, do {} situps!", simulated_expensive_calculation(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", simulated_expensive_calculation(intensity));
        }
    }
}
```

- 优化点1: 在`intensity`小于25时,`simulated_expensive_calculation`被调用了2次,可以优化为1次

```rust
use std::thread;
use std::time::Duration;

fn main() {
  let simulated_user_specified_value = 10;
  let simulated_random_number = 7;

  generate_workout(simulated_user_specified_value, simulated_random_number);
}

// 本函数用于模拟一个耗时的计算过程
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

// 本函数用于模拟生成健身计划的过程
fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_result = simulated_expensive_calculation(intensity);
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result);
        println!("Next, do {} situps!", expensive_result);
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", simulated_expensive_calculation(intensity));
        }
    }
}
```

- 优化点2: 在`intensity`大于等于25时,调用`simulated_expensive_calculation()`的过程也可以使用`expensive_result`来代替

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    
    generate_workout(simulated_user_specified_value, simulated_random_number);
}

// 本函数用于模拟一个耗时的计算过程
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

// 本函数用于模拟生成健身计划的过程
fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_result = simulated_expensive_calculation(intensity);
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result);
        println!("Next, do {} situps!", expensive_result);
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result);
        }
    }
}
```

- 优化后的问题: 在`random_number`值为3的分支中,其实没有调用`simulated_expensive_calculation()`函数,但是`expensive_result`变量仍然被计算了

本质上我们要的效果是:**仅在需要`simulated_expensive_calculation()`函数的结果时才进行计算**

- 使用闭包来延迟计算的过程,确保只在需要时才进行计算

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

// 本函数用于模拟一个耗时的计算过程
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

// 本函数用于模拟生成健身计划的过程
fn generate_workout(intensity: u32, random_number: u32) {

    // 此处如果只定义闭包而不调用这个闭包 编译器会报错 因为无法推断num的类型
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

这里你可能会想到,在`intensity`小于25时,仍然调用了2次闭包.一个比较简单的解决方案是,将闭包的结果保存到一个变量中,然后在需要时使用这个变量

但可以利用闭包的特性,有其他的解决方案.这个后续再讲