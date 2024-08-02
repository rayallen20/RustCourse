# PART1. 读取文件

- `std::fs`:该模块用于文件系统操作

```rust
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query :String = args[1].clone();
    let filename :String = args[2].clone();

    println!("Search for {}", query);
    println!("In file {}", filename);

    let content :String = fs::read_to_string(filename).
        expect("Something went wrong reading the file");
    println!("With text:\n{}", content)
}
```

```
cargo run a ./poem.txt
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/minigrep a ./poem.txt`
Search for a
In file ./poem.txt
With text:
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!
- Emily Dickinson
```