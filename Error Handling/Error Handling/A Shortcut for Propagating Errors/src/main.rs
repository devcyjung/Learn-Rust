use fs::File;
use io::Read;
use std::{fs, io};

const VIMRC: &str = "/home/rhel/.vimrc";
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open(VIMRC)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open(VIMRC)?.read_to_string(&mut s)?;

    Ok(s)
}

fn read_username_from_file_3() -> Result<String, io::Error> {
    fs::read_to_string(VIMRC)
}

fn main() {
    // put you code here to launch it
    let res1 = read_username_from_file();
    let res2 = read_username_from_file_2();
    let res3 = read_username_from_file_3();
    println!("{:?}", res1);
    println!();
    println!("{:?}", res2);
    println!();
    println!("{:?}", res3);
    println!();
}
