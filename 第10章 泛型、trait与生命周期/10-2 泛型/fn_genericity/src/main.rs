use std::cmp::PartialOrd;

fn main() {
    let numbers1 = vec![34, 50, 25, 100, 65];
    println!("The largest number in numbers1 is {}", find_largest(&numbers1));

    // 如果将集合替换为字符切片,即寻找字符切片中的最大字符
    // 这个场景下就可以使用泛型来实现
    let chars1 = vec!['y', 'm', 'a', 'q'];
    println!("The largest char in chars1 is {}", find_largest(&chars1));
}

fn find_largest<T: PartialOrd> (numbers: &[T]) -> T {
    let mut largest = numbers[0];

    for &number in numbers {
        if number > largest {
            largest = number;
        }
    }

    largest
}
