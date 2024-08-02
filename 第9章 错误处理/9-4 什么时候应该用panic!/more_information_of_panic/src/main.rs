use std::net::IpAddr;

fn main() {
    // 这行代码绝对不会panic 因为127.0.0.1是一个有效的IP地址
    // 但编译器不知道这一点 所以这里我们可以写一个unwrap() 但实际上我们自己知道这里绝对不会panic
    let home: IpAddr = "127.0.0.1".parse().unwrap();
}
