fn main() {
    let mut counter = 0;

    loop {
        println!("Hello, world {}!", counter);

        match counter {
            10 => break,
            5 => println!("Halfway there!"),
            _ => (),
        }

        counter += 1;
    }
}
