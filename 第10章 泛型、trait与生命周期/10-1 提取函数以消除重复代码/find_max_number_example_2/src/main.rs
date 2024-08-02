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