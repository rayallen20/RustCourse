# PART1. 从crates.io安装二进制crate

- 命令: `cargo install <crate-name>`
- 来源: crates.io
- 限制: 只能安装具有二进制目标(binary target)的crate

二进制目标binary target:是一个可运行程序

- 要么由拥有`src/main.rs`的crate生成
- 要么由其他被指定为二进制的入口文件的crate生成

通常,一个crate的README中有关于crate的描述:

- 是否拥有library target
- 是否拥有binary target
- 也有可能两者都有

# PART2. cargo install

- `cargo install`命令安装的二进制存放在`/bin`目录下
- 如果你使用的是`rustup`安装的Rust,切没有任何自定义配置,那么二进制存放目录为`$HOME/.cargo/bin`
  - 为了确保这个二进制能够直接运行,需要将这个目录添加到`$PATH`环境变量中

例:

```
cargo install rust_tutorials
    Updating crates.io index
  Downloaded rust_tutorials v0.1.0
  Downloaded 1 crate (807 B) in 1.28s
  Installing rust_tutorials v0.1.0
    Updating crates.io index
   Compiling rust_tutorials v0.1.0
    Finished `release` profile [optimized] target(s) in 5.28s
  Installing /.cargo/bin/rust_tutorials
   Installed package `rust_tutorials v0.1.0` (executable `rust_tutorials`)
```

```
rust_tutorials 
Hello, world!
```

```
~ % ls ~/.cargo/bin|grep rust_tutorials
rust_tutorials
~ % file ~/.cargo/bin/rust_tutorials 
~/.cargo/bin/rust_tutorials: Mach-O 64-bit executable x86_64
```

# PART3. 使用自定义命令来扩展cargo

- cargo被设计成可以使用子命令来扩展的工具

例: 如果`$PATH`中的某个二进制文件名为`cargo-xxx`,那么`cargo xxx`就会调用这个二进制文件

这样从使用上来看,就像`xxx`是`cargo`的一个子命令一样

类似这样的自定义命令可以通过`cargo --list`来列出

优点:可使用`cargo install`来安装扩展,然后像内置工具一样运行该扩展