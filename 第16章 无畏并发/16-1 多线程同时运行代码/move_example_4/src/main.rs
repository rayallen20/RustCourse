use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let join_handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    drop(v);  // error: value used here after move

    join_handle.join().unwrap();
}
