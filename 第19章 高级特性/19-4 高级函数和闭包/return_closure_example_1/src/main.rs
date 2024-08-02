fn return_closure() -> Fn(i32) -> i32 {     // error: doesn't have a size known at compile-time
    |x| x + 1
}

fn main() {
}
