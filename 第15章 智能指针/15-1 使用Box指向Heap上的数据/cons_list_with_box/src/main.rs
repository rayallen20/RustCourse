#[derive(Debug)]
enum Cons {
    Cons(i32, Box<Cons>),
    Nil,
}

fn main() {
    let c = Cons::Cons(
        1, Box::new(
            Cons::Cons(
                2, Box::new(
                    Cons::Nil
                )
            )
        )
    );

    println!("{:?}", c)
}
