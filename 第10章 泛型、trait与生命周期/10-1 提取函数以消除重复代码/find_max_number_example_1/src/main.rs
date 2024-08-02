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
