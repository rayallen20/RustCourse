fn main() {
    let r = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");
    &s // error: missing lifetime specifier
} // 此时s离开作用域,数据会被销毁.但是返回的引用还是指向这块数据,因此出现了悬空引用