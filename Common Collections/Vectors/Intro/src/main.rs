use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() {
    let file_names = [
        ".config/nvim/init.lua",
        ".vimrc",
        ".vim_runtime/vimrcs/basic.vim",
        ".vim_runtime/vimrcs/filetypes.vim",
        ".vim_runtime/vimrcs/plugins_config.vim",
        ".vim_runtime/vimrcs/extended.vim",
        ".vim_runtime/my_configs.vim",
    ];

    for file_name in file_names {
        handle_file_name(file_name);
    }
}

fn vector_file(home_path: &str) -> Result<Vec<String>, io::Error> {
    let mut path = dirs::home_dir().unwrap();
    path.push(home_path);

    println!("Reading file at {:?}...", path);

    let file = BufReader::new(File::open(path)?);
    let mut vec: Vec<String> = Vec::new();

    for line in file.lines() {
        match line {
            Ok(line) => vec.push(line.to_string()),
            Err(why) => return Err(why),
        }
    }

    Ok(vec)
}

fn handle_file_name(filename: &str) {
    let file = vector_file(filename);

    match file {
        Err(why) => println!("Error opening ~/{}: {}", filename, why),
        Ok(vec) => {
            println!("Displaying content of ~/{}", filename);
            for (ln_content, ln_no) in vec.iter().zip(1..) {
                println!("Line {:04}: {}", ln_no, ln_content);
            }
            println!("~/{}, line count: {}", filename, vec.len());
        }
    }
}
