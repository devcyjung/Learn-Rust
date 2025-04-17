use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    // Using the move keyword to force a closure
    // to take ownership of the values it uses
    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });

    // drop(v);
    handle.join().expect("failed to join child thread");
}
