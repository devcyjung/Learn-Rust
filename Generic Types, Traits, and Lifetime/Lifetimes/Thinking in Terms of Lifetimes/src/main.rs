fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

fn longest2<'a>(x: &str, y: &str) -> &'a str {
    // let result = String::from("really long string");
    // !!! ERROR: cannot return value referencing local variable `result`
    // result.as_str()
    "really long string"
}

fn main() {
    // put you code here to launch it
    println!("{}", longest("abcde", "ab"));
    println!("{}", longest2("abcde", "ab"));
}
