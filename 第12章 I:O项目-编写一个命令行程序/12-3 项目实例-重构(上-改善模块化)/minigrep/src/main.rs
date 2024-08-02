use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::new(args);

    println!("Search for {}", config.query);
    println!("In file {}", config.filename);

    let content :String = fs::read_to_string(config.filename).
        expect("Something went wrong reading the file");
    println!("With text:\n{}", content)
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: Vec<String>) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();
        Config {
            query,
            filename,
        }
    }
}