use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file(filepath: &str) -> Result<String, io::Error> {
    let f = File::open(filepath);

    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    let mut f = f?;

    let mut s = String::new();

    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    // put you code here to launch it
    for fp in ["hello.txt", "/home/rhel", "/home/rhel/.vimrc"] {
        let res = read_username_from_file(fp)
            .unwrap_or_else(|err| format!("Failed reading file due to error: {:?}", err));
        println!("Path: {}", fp);
        println!("{}", res);
        println!();
    }
}
