type Closure = fn(i32) -> i32;

fn return_closure() -> Closure {
    |x| x + 1
}

fn main() {
    let f: Closure = return_closure();
    println!("{}", f(1));
}
