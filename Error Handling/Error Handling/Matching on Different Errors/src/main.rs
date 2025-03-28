use std::fs::File;
use std::io::ErrorKind;

const HOME: &str = "/home/rhel/";
const RUST_ROVER: &str = "RustroverProjects/";
const MATCHING_ERROR_KINDS: &str =
    "Learn Rust/Error Handling/Error Handling/Matching on Different Errors/";
const FILE_PATHS: [&str; 5] = [
    "/var/log/secure",
    HOME,
    constcat::concat!(HOME, ".config/nvim/init.lua"),
    constcat::concat!(HOME, RUST_ROVER, MATCHING_ERROR_KINDS, "Cargo.toml"),
    constcat::concat!(HOME, RUST_ROVER, MATCHING_ERROR_KINDS, "random.txt"),
];
fn main() {
    for filepath in FILE_PATHS {
        println!("Path: {}", filepath);
        let file = match File::open(filepath) {
            Err(e) if e.kind() == ErrorKind::NotFound => {
                println!("File {} not found", filepath);
                continue;
            }
            Err(e) => {
                println!("Failed to open file {}: {:?}", filepath, e.kind());
                continue;
            }
            Ok(file_read) => file_read,
        };
        let meta = match file.metadata() {
            Err(e) => {
                println!("Failed to read metadata from {:?}: {:?}", filepath, e);
                break;
            }
            Ok(m) => {
                println!("{:?}", m);
                m
            }
        };
        let ftype = meta.file_type();
        match ftype {
            f if f.is_file() => {
                println!("FileType: File");
            }
            f if f.is_dir() => {
                println!("FileType: Dir");
            }
            f if f.is_symlink() => {
                println!("FileType: Symlink");
            }
            f => {
                println!("FileType: Other");
                println!("{:?}", f);
            }
        }
        let permissions = meta.permissions();
        match permissions.readonly() {
            true => {
                println!("FilePermission: Readonly");
            }
            false => {
                println!("FilePermission: Not readonly");
            }
        }
        println!("len: {}", meta.len());
        let modified_time = match meta.modified() {
            Err(e) => {
                println!("Failed to read modified time from {:?}: {:?}", filepath, e);
                break;
            }
            Ok(m) => m,
        };
        let accessed_time = match meta.accessed() {
            Err(e) => {
                println!("Failed to read accessed time from {:?}: {:?}", filepath, e);
                break;
            }
            Ok(m) => m,
        };
        let created_time = match meta.created() {
            Err(e) => {
                println!("Failed to read created time from {:?}: {:?}", filepath, e);
                break;
            }
            Ok(m) => m,
        };
        for t in [created_time, modified_time, accessed_time] {
            println!("{:?} ago", t.elapsed());
        }
        if !(created_time <= modified_time && modified_time <= accessed_time) {
            println!(
                "Somehow not in created -> modified -> accessed order, {}",
                filepath
            );
        }
    }
}
