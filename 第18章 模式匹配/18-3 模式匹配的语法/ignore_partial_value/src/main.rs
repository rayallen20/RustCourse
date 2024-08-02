fn main() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    // 本例中match表达式匹配的是一个元组 该元组的类型为 (Option<i32>, Option<i32>)
    // match表达式匹配实现了Copy Trait的类型时,不会获取数据的所有权,而是复制数据
    // match表达式如果匹配引用,则不会获取数据的所有权
    // 但是如果match表达式匹配的类型没有实现Copy Trait,则会获取数据的所有权
    // 本例中,setting_value和new_setting_value都是Option<i32>类型,没有实现Copy Trait
    // 所以match表达式会获取数据的所有权
    match (setting_value, new_setting_value) {
        // 匹配元组中2个元素的值均为Some变体的模式
        // 但该模式并不关心Some中的值是什么 因此可以使用_忽略模式中的部分值
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }

        // 匹配元组中2个元素的值不全是Some变体的模式
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);
}
