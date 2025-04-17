use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    println!("program initialized");
    let (tx, rx) = mpsc::channel::<&str>();
    let handle = thread::spawn(move || {
        thread::sleep(Duration::from_secs(3));
        if tx.send("Hello main thread").is_ok() {
            println!("message sent !");
        }
    });
    println!("main thread waiting");
    if let Ok(msg) = rx.recv() {
        if handle.join().is_ok() {
            println!("thread exited successfully with message {msg}");
        }
    }
    println!("main thread exiting");
}
