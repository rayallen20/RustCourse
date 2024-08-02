use std::env;

fn main() {
    // 此处必须指定args的类型 因为collect()方法返回的类型为一个 实现了 FromIterator trait 的泛型
    // 因此编译器无法推断其具型 必须手动指定
    // 需要注意的是 env::args()函数只能接收Unicode字符
    // 若想要能够接收非法的Unicode字符 可以使用std::env::args_os()函数
    let args: Vec<String> = env::args().collect();

    // 第1个参数为要搜索的内容
    let query :String = args[1].clone();

    // 第2个参数为要搜索的文件
    let filename :String = args[2].clone();

    println!("Search for {}", query);
    println!("In file {}", filename);
}
