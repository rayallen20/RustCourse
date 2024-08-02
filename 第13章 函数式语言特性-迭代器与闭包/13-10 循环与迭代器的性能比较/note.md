# PART1. 测试集的性能评估

测试上一节优化后,使用迭代器的版本与使用for循环的版本时,使用了一本小说,测试结果显示迭代器的版本反而更快一点.

# PART2. 零开销抽象

在Rust中,迭代器是一种高层次的抽象.但是在编译后,它生成的代码与使用for循环的代码几乎是一样的.这意味着,在Rust中使用迭代器并不会引入任何运行时开销.

而这套高层次的抽象就叫零开销抽象(Zero-Cost Abstraction).

它们是零开销的,这意味着它们不会引入任何运行时开销.这是因为Rust的迭代器是基于trait的,而trait是一种零开销的抽象.

# PART3. 音频解码器的例子

我们来看一个例子,这个例子是一个音频解码器.我们将使用迭代器来实现这个解码器.

```rust
let buffer: &mut [i32];
let coefficients: [i64; 12];
let qlp_shift: i16;

for i in 12..buffer.len() {
    let prediction = coefficients.iter()
        .zip(&buffer[i - 12..i])
        .map(|(&c, &s)| c * s as i64)
        .sum::<i64>() >> qlp_shift;
    let delta = buffer[i];
    buffer[i] = prediction as i32 + delta;
}
```

主要看prediction的计算,它是一个迭代器,它的计算过程是这样的:

- `coefficients.iter()`: 基于一个长度为12的数组创建一个迭代器
- `zip(&buffer[i - 12..i])`: 将这个迭代器与buffer的一个切片进行zip操作,这个切片的长度也是12
  - 实际上这里就相当于进行了12次迭代

但实际上这段代码最终生成的汇编代码,和手写循环的汇编几乎是一样的.

因为Rust的编译器能够知道这里要循环12次,所以使用了一个"展开策略".可以认为就是把这段代码展开复制了12次,这样反而能消除循环控制语句带来的性能开销.

因此,在Rust中,应尽可能使用迭代器,而不是手写循环.这样既能让代码在感官上保持高层次的抽象,又能保证性能.