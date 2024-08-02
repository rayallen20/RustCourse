use std::collections::HashMap;

fn main() {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    // zip() 方法创建一个元组的迭代器,其中新的迭代器将同时包含 teams 和 initial_scores 中的元素
    let team_scores_tuple = teams.iter().zip(initial_scores.iter());
    let team_scores_tuple_clone = team_scores_tuple.clone();

    // 在Rust中 迭代器是一种消费者
    // 遍历迭代器的操作会"消费"迭代器中的元素,这意味着迭代器会获取元素的所有权并在遍历过程中将其移出
    // 这里就将 team_scores_tuple 这个迭代器中的所有元素全部移出了
    // 因此这里需要事前准备一个克隆 用于后续的操作
    for ele in team_scores_tuple {
        println!("{:?}", ele); // ("Blue", 10) ("Yellow", 50)
    }

    // 使用 collect() 方法将元组转换为 HashMap
    // collect()方法可以将数据转换为多种不同的集合类型 所以需要指定类型
    // 这里的_是占位符,表示HashMap的键和值的类型是由collect()方法推断出来的
    let team_scores: HashMap<_, _> = team_scores_tuple_clone.collect();
    println!("{:?}", team_scores); // {"Blue": 10, "Yellow": 50}
}
