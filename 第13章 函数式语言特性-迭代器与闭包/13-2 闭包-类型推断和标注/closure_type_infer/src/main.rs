fn main() {
    // 仅定义闭包而不使用闭包的情况下 无法推断x的类型 会报错
    let example_closure = |x| x;

    // 若此时调用闭包 则可以推断x的类型 x的类型一旦被推断出来 就不能再改变
    let s = example_closure(String::from("hello"));

    // 此时再使用不同类型的参数调用闭包 会报错
    let n = example_closure(5);
}
