# PART1. Rust错误处理概述

大部分情况下,Rust在编译时就提示错误,并提示你处理

Rust的错误分类:

- 可恢复错误
  - 例如:文件未找到/网络连接失败等错误 这种错误一般会返回给用户,并允许用户再次尝试
- 不可恢复错误
  - 不可恢复错误通常就是bug,例如索引越界

Rust的错误处理方式:

- 针对可恢复错误: Result<T,E>
- 针对不可恢复错误: panic!

# PART2. 不可恢复的错误与panic!

默认情况下,当`panic!`宏被调用时,会有如下操作:

- 程序打印错误信息
- 展开(unwind)、清理调用栈
- 退出程序

# PART3. 为应对`panic!`,展开或中止(abort)调用栈

默认情况下,当panic发生时:

- 程序展开调用栈(这个工作量很大)
  - 因为Rust需要沿着调用栈往回走
  - 在往回走的过程中,清理每一个遇到的函数中的数据

- 而另外一种可选的操作,就是立即中止调用栈:
  - 不进行清理动作,直接停止程序
  - 这种方式会快很多,但是程序使用的内存需要由OS进行清理和释放

如果你想让你的二进制文件更小,就可以将设置从默认的"展开"修改为"中止"

- 在`Cargo.toml`中添加如下配置:

```toml
[profile.release]
panic = 'abort'
```

例:

```toml
[package]
name = "set_abort_panic"
version = "0.1.0"
edition = "2021"

[dependencies]

# 本部分设置仅在debug模式下生效
# [profile.debug]

# 本部分设置仅在release模式下生效
[profile.release]
# 在release模式下,当panic时直接终止程序
panic = "abort"
```

# PART4. `panic!`的例子

```rust
fn main() {
    panic!("crash and burn")
}
```

```bash
 cargo run
   Compiling panic_example v0.1.0 (/panic_example)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 6.09s
     Running `target/debug/panic_example`
thread 'main' panicked at src/main.rs:2:5:
crash and burn
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

这个例子中,`panic!`发生在我们自己写的代码中.再来看一个例子:

```rust
fn main() {
    let v = vec![1, 2, 3];
    v[99];
}
```

```bash
cargo run
   Compiling panic_example_2 v0.1.0 (/panic_example_2)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.43s
     Running `target/debug/panic_example_2`
thread 'main' panicked at src/main.rs:3:6:
index out of bounds: the len is 3 but the index is 99
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

panic的信息如下:

```
thread 'main' panicked at src/main.rs:3:6:
index out of bounds: the len is 3 but the index is 99
```

那么问题来了,这里我们并没有调用`panic!`宏,这个panic是谁调用的呢?

# PART5. 使用`RUST_BACKTRACE`环境变量

注意错误信息中的:note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

我们可以通过设置`RUST_BACKTRACE`环境变量来查看调用栈信息

```bash
RUST_BACKTRACE=1 && cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.06s
     Running `target/debug/panic_example_2`
thread 'main' panicked at src/main.rs:3:6:
index out of bounds: the len is 3 but the index is 99
stack backtrace:
   0: rust_begin_unwind
             at /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/panicking.rs:645:5
   1: core::panicking::panic_fmt
             at /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/panicking.rs:72:14
   2: core::panicking::panic_bounds_check
             at /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/panicking.rs:209:5
   3: <usize as core::slice::index::SliceIndex<[T]>>::index
             at /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/index.rs:264:10
   4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
             at /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/index.rs:18:9
   5: <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index
             at /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:2829:9
   6: panic_example_2::main
             at ./src/main.rs:3:6
   7: core::ops::function::FnOnce::call_once
             at /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

panic!可能出现在:

- 我们自己写的代码中
- 我们所依赖的代码中

可通过调用panic!的函数的回溯信息(backtrace)来定位问题

注意这句提示:note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

# PART6. `RUST_BACKTRACE`环境变量的取值

`RUST_BACKTRACE`环境变量有三个取值:

- `0`: 不显示调用栈信息
- `1`: 显示调用栈信息
- `full`: 显示详细的调用栈信息

```bash
export RUST_BACKTRACE=full && cargo run
   Compiling panic_example_2 v0.1.0 (/panic_example_2)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.57s
     Running `target/debug/panic_example_2`
thread 'main' panicked at src/main.rs:3:6:
index out of bounds: the len is 3 but the index is 99
stack backtrace:
   0:        0x105d81c13 - std::backtrace_rs::backtrace::libunwind::trace::h5bfe4001d39ebff3
                               at /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/../../backtrace/src/backtrace/libunwind.rs:105:5
   1:        0x105d81c13 - std::backtrace_rs::backtrace::trace_unsynchronized::h86a1158ecfd606b7
                               at /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:        0x105d81c13 - std::sys_common::backtrace::_print_fmt::h0916a318c0fc97ff
                               at /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/sys_common/backtrace.rs:68:5
   3:        0x105d81c13 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h9e09877065e04f77
                               at /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/sys_common/backtrace.rs:44:22
   4:        0x105d9a6db - core::fmt::rt::Argument::fmt::h9d2968fcafab28ba
                               at /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/fmt/rt.rs:142:9
   5:        0x105d9a6db - core::fmt::write::ha1eda037e545f7da
                               at /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/fmt/mod.rs:1153:17
   6:        0x105d7febe - std::io::Write::write_fmt::h982a70ce7879c870
                               at /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/io/mod.rs:1843:15
   7:        0x105d81a01 - std::sys_common::backtrace::_print::h0896aee7ef2a1b71
                               at /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/sys_common/backtrace.rs:47:5
   8:        0x105d81a01 - std::sys_common::backtrace::print::h614a07b8de11add7
                               at /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/sys_common/backtrace.rs:34:9
   9:        0x105d82d69 - std::panicking::default_hook::{{closure}}::h079cbc758586c627
  10:        0x105d82ad0 - std::panicking::default_hook::ha4241b247d28f540
                               at /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/panicking.rs:292:9
  11:        0x105d8379e - std::panicking::rust_panic_with_hook::h610a5d47d992d59c
                               at /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/panicking.rs:779:13
  12:        0x105d83104 - std::panicking::begin_panic_handler::{{closure}}::hb7bd4ff0d901d687
                               at /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/panicking.rs:657:13
  13:        0x105d820e9 - std::sys_common::backtrace::__rust_end_short_backtrace::h08de5c5c123a7cab
                               at /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/sys_common/backtrace.rs:171:18
  14:        0x105d82e36 - rust_begin_unwind
                               at /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/panicking.rs:645:5
  15:        0x105d9ff65 - core::panicking::panic_fmt::h2c2c8066e5becbb9
                               at /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/panicking.rs:72:14
  16:        0x105da0146 - core::panicking::panic_bounds_check::h96dd81f5e759c901
                               at /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/panicking.rs:209:5
  17:        0x105d64546 - <usize as core::slice::index::SliceIndex<[T]>>::index::h75d499b6127be37c
                               at /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/index.rs:264:10
  18:        0x105d64636 - core::slice::index::<impl core::ops::index::Index<I> for [T]>::index::h0459d6eece52e9f1
                               at /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/index.rs:18:9
  19:        0x105d64636 - <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index::h446b26c57f18941c
                               at /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:2829:9
  20:        0x105d64ebb - panic_example_2::main::hf3a3abfdcd65a62c
                               at /panic_example_2/src/main.rs:3:6
  21:        0x105d64fce - core::ops::function::FnOnce::call_once::h450a8e72995151cc
                               at /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ops/function.rs:250:5
  22:        0x105d64741 - std::sys_common::backtrace::__rust_begin_short_backtrace::h7b45c8227a9d6004
                               at /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/sys_common/backtrace.rs:155:18
  23:        0x105d646c4 - std::rt::lang_start::{{closure}}::h192db0ce983ba036
                               at /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/rt.rs:166:18
  24:        0x105d7e4e0 - core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &F>::call_once::he0a4f6383506aade
                               at /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ops/function.rs:284:13
  25:        0x105d7e4e0 - std::panicking::try::do_call::h0f1c0756fb36e3b9
                               at /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/panicking.rs:552:40
  26:        0x105d7e4e0 - std::panicking::try::h892374a57e1b10f8
                               at /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/panicking.rs:516:19
  27:        0x105d7e4e0 - std::panic::catch_unwind::hed18309e55f688fa
                               at /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/panic.rs:146:14
  28:        0x105d7e4e0 - std::rt::lang_start_internal::{{closure}}::had3c7cc5a0ca3f18
                               at /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/rt.rs:148:48
  29:        0x105d7e4e0 - std::panicking::try::do_call::h71ab2f8767c558bd
                               at /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/panicking.rs:552:40
  30:        0x105d7e4e0 - std::panicking::try::h27002495ef851327
                               at /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/panicking.rs:516:19
  31:        0x105d7e4e0 - std::panic::catch_unwind::h9c2e42db96b1f0de
                               at /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/panic.rs:146:14
  32:        0x105d7e4e0 - std::rt::lang_start_internal::h23f2a7c89aeb33c4
                               at /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/rt.rs:148:20
  33:        0x105d64697 - std::rt::lang_start::hd80a900ba346df79
                               at /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/rt.rs:165:17
  34:        0x105d64f28 - _main
  35:     0x7ff815cac41f - <unknown>
```

# PART7. 启用调试符号

`cargo build`或`cargo run`命令中,不使用`--release`参数,即为debug模式,此时编译器会生成调试符号