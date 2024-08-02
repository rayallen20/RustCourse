# PART1. 提取函数以消除重复代码

现有一段代码,用于实现从一个Vector中寻找最大值,并将其打印出来.代码如下:

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let mut largest = numbers[0];

    for number in &numbers {
        if *number > largest {
            largest = *number;
        }
    }

    println!("The largest number is {}", largest);
}
```

现在,我们需要从另一个Vector中寻找最大值,并将其打印出来.代码如下:

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let mut largest = numbers[0];

    for number in &numbers {
        if *number > largest {
            largest = *number;
        }
    }
    println!("The largest number is {}", largest);

    let numbers = vec![10, 20, 30, 40, 50];
    let mut largest = numbers[0];
    for number in &numbers {
        if *number > largest {
            largest = *number;
        }
    }
    println!("The largest number is {}", largest);
}
```

那么,很明显可以将"寻找最大值"这个操作提取出来,写成一个函数,以消除重复代码.代码如下:

```rust
fn main() {
    let numbers1 = vec![34, 50, 25, 100, 65];
    let numbers2 = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    // 此处调用 find_largest()函数时, 传入的参数类型为 &Vec<i32>,而形参的类型为 &[i32]
    // 这样可以成功通过编译的原因是因为Vec<T>实现了Deref trait(解引用强制转换),可以将 &Vec<T> 转换为 &[T]
    println!("The largest number in numbers1 is {}", find_largest(&numbers1));
    println!("The largest number in numbers2 is {}", find_largest(&numbers2));
}

fn find_largest(numbers: &[i32]) -> i32 {
    let mut largest = numbers[0];

    // for number in numbers {
    //     if *number > largest {
    //         largest = *number;
    //     }
    // }

    // &number中的&符不是引用符号,而是表示模式匹配
    // 该符号会匹配numbers中每一个元素的引用 并立即解引用
    // 然后将解引用的值赋给number
    // 这样做的好处在于不会在循环中有大量的 *number 出现
    for &number in numbers {
        if number > largest {
            largest = number;
        }
    }

    largest
}
```

# PART2. 消除重复代码的步骤

- 识别重复代码
- 提取重复代码到函数体中,并在函数签名中指定函数的输入和返回值
- 将重复的代码使用函数调用进行替代
