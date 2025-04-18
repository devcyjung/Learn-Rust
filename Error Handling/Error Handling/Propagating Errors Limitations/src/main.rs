use std::error::Error;
use std::fs::File;

// !!! ERROR: the `?` operator can only be used
// in a function that returns `Result` or `Option`
// fn main() {
//     let f = File::open("hello.txt")?;
// }

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("/home/rhel/.vimrc")?;
    println!("FILE: {f:?}");
    let f = File::open("hello.txt")?; // main() returns here.
    println!("FILE: {f:?}");
    let f = File::open("/home/rhel/.vimrc")?;
    println!("FILE: {f:?}");
    Ok(())
}
