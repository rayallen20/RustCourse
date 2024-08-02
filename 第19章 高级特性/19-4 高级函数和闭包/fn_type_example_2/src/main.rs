fn main() {
    println!("Hello, world!");
}

fn to_string_1(v: Vec<i32>) -> Vec<String> {
    v.iter()
        // 此处使用闭包 x的类型为&i32 返回值类型为String
        .map(|x| x.to_string())
        .collect()
}

fn to_string_2(v: Vec<i32>) -> Vec<String> {
    v.iter()
        // 此处使用函数指针 ToString是一个trait 该trait定义了to_string()方法
        // 所以此处传递的是一个函数指针
        .map(ToString::to_string)
        .collect()
}