type Closure = Box<dyn Fn(i32) -> i32>;

fn return_closure() -> Closure {
    Box::new(|x| x + 1)
}

fn main() {
    let f: Closure = return_closure();
    println!("{}", f(1));
}
