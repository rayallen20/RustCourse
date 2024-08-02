// 使用static关键字声明全局变量
// 全局变量要求使用大写字母和下划线组成,且必须声明类型
// 静态变量只能存储拥有'static生命周期的引用 ('static生命周期是整个程序运行期间的生命周期)
// 这也就意味着编译器能够推断出全局变量的生命周期 无需手动标注
// 因此无需写成 static HELLO_WORLD: &'static str = "Hello, world!"; 这样的形式
static HELLO_WORLD: &str = "Hello, world!";

fn main() {
    println!("{}", HELLO_WORLD);
}
