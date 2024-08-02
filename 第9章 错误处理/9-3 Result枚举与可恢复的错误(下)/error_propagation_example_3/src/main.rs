use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    let username = read_username_from_file();
    match username {
        Ok(s) => println!("The username is: {}", s),
        Err(e) => println!("Error: {}", e),
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
