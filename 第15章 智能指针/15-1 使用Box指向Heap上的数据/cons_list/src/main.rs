#[derive(Debug)]
enum Cons {
    Cons(i32, Cons),
    Nil,
}

fn main() {
    let c = Cons::Cons(1, Cons::Cons(2, Cons::Cons(3, Cons::Nil)));
    println!("{:?}", c);
}
