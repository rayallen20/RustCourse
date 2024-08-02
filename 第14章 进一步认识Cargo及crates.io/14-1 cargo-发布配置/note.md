# PART1. 通过release profile来自定义构建

## 1.1 release profile

- 是一系列预定义的配置方案
- 可自定义:可使用不同的配置,对代码编译拥有更多的控制
- 每个profile(配置档案)都独立于其他的profile

cargo主要的2个profile:

- dev profile: 适用于开发,即`cargo build`
- release profile: 适用于发布,即`cargo build --release`

例:

```
cargo build
   Compiling project_example v0.1.0 (/project_example)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.53s
```

可以看到,`dev` profile是默认的,`unoptimized + debuginfo`表示未优化和包含调试信息

```
cargo build --release
   Compiling project_example v0.1.0 (/project_example)
    Finished `release` profile [optimized] target(s) in 0.62s
```

可以看到,`release` profile是优化的,`optimized`表示已优化

# PART2. 自定义profile

- 针对每个profile,cargo都提供了默认的配置
- 如果想自定义某个profile(dev或release)的配置:
  - 在`Cargo.toml`中添加`[profile.xxx]`部分,其中`xxx`是profile的名称,如`dev`或`release`
  - 在这个部分中覆盖默认配置的子集即可
  - 通常不会覆盖全部配置,仅覆盖想修改的部分即可

例:

```toml
[package]
name = "project_example"
version = "0.1.0"
edition = "2021"

[dependencies]
```

该配置文件中没有`[profile]`部分,即表示使用默认配置

`opt-level`: 表示Rust在编译的时候对代码进行优化的程度,默认是0,即不优化.最大值为3,表示最大优化

但优化程度越高,编译所需的时间也越长

```toml
[package]
name = "project_example"
version = "0.1.0"
edition = "2021"

[dependencies]

[profile.dev]
opt-level = 1

[profile.release]
opt-level = 3
```

修改`Cargo.toml`后再次编译:

```
cargo build          
   Compiling project_example v0.1.0 (/project_example)
    Finished `dev` profile [optimized + debuginfo] target(s) in 0.57s
```

注意和修改前的编译结果对比:

```
cargo build
Compiling project_example v0.1.0 (/project_example)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.53s
```

可以看到将`opt-level`设置为1后,编译结果为`optimized`而不是`unoptimized`

每个配置的默认值和完整选项可[参见此处](https://doc.rust-lang.org/cargo/index.html)