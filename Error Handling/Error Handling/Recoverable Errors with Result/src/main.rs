use std::{
    fs::File,
    io::{Error, Read},
};

struct FileWithPath<'a> {
    path: &'a str,
    file: Result<File, Error>,
}

fn main() {
    let fname = [
        "/home/rhel/.config/nvim/init.lua",
        "/var/log/secure",
        "/run/udev/data/+acpi:PNP0103:00",
        "/usr/bin/firefox",
        "/home/rhel",
    ];

    let files = fname.map(|path| FileWithPath {
        path,
        file: File::open(path),
    });

    for res in files {
        let mut buf = Vec::<u8>::new();
        println!("Opening {}", res.path);
        match res.file {
            Ok(mut f) => {
                let ok = f.read_to_end(&mut buf);
                match ok {
                    Ok(n) => {
                        println!("Read {} = {} bytes", buf.len(), n);
                        println!("{:?}", f.metadata());
                        println!();
                    }
                    Err(e) => {
                        println!("Failed to read from file {:?}", f.metadata());
                        println!("Error: {}", e);
                        println!("Kind: {}", e.kind());
                        println!();
                    }
                }
            }
            Err(e) => {
                println!("Failed to open file {}", e);
                println!("Kind: {}", e.kind());
                println!();
            }
        }
    }
}
