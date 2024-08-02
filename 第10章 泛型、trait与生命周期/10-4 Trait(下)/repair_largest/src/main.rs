fn main() {
    let numbers1 = vec![34, 50, 25, 100, 65];
    println!("The largest number in numbers1 is {}", find_largest(&numbers1));

    let chars1 = vec!['y', 'm', 'a', 'q'];
    println!("The largest char in chars1 is {}", find_largest(&chars1));

    let strings1: Vec<String> = vec!["hello".to_string(), "world".to_string()];
    println!("The largest string in strings1 is {}", find_largest(&strings1));
}

fn find_largest<T: PartialOrd + Clone> (numbers: &[T]) -> T {
    let mut largest = numbers[0].clone();

    for number in numbers.iter() {
        // number的类型为&T 所以这里需要解引用
        if *number > largest {
            // 赋值时同样也是为了避免所有权的移动 将number的clone赋值给largest
            largest = number.clone();
        }
    }

    largest
}