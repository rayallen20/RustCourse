use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query :String = args[1].clone();
    let filename :String = args[2].clone();

    println!("Search for {}", query);
    println!("In file {}", filename);

    let content :String = fs::read_to_string(filename).
        expect("Something went wrong reading the file");
    println!("With text:\n{}", content)
}
