fn main() {
    // some_string的值(也就是数据hello)的所有权被移动到s1
    let s1 = gives_ownership();

    let s2 = String::from("hello");

    // step1. s2的值(也就是数据hello)的所有权被移动到函数内部
    // step3. s3获得数据hello的所有权
    let s3 = takes_and_gives_back(s2);

    // println!("{}", s2) // error: value borrowed here after move
} // step4. s1和s3离开作用域,调用drop函数,释放内存; s2的值的所有权已经被移动,所以不会发生任何事情

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
} // some_string作为返回值,所有权移动到调用者,本例中是s1

fn takes_and_gives_back(a_string: String) -> String {
    a_string
} // step2. takes_and_gives_back函数获得hello的所有权,并将所有权返回给调用者,本例中是s3
