fn main() {
    let numbers = (1, 2, 3, 4, 5);

    match numbers {
        // 使用_忽略元组中的第2个和第4个元素 仅匹配第1个 第3个和第5个元素
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth);
        }
    }
}
