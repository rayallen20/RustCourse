fn main() {
    let x = 5;
    let y = Box::new(5);

    // 使用解引用操作符获取Box指向的数据
    // 对Box<T>类型的值使用解引用操作符*会得到一个T类型
    assert_eq!(x, *y);

    let w = &6;
    let y = Box::new(6);
    // assert_eq!宏在比较2个引用时,比较的是引用的值是否相等
    // 这里不能直接拿w和y比较,因为虽然它们都是指向6的引用,但w的类型为&i32,y的类型为Box<i32>
    // 所以需要先解引用y,再和w比较(实际上这里最后取y的引用只是为了让y和w的类型一致)
    assert_eq!(w, &(*y));
}
