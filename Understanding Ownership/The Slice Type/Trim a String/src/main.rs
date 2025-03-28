use trim_a_string::trimmed_space;

fn main() {
    let str_literal = "   Rust   ";
    println!("{}", trimmed_space(str_literal));

    let string = String::from("  Rust  ");
    println!("{}", trimmed_space(&string[..]));
    
    let str = "   ";
    println!("{}", trimmed_space(str));

    let str = String::from("            R");
    println!("{}", trimmed_space(&str));
    let str = String::from("R           R");
    println!("{}", trimmed_space(&str));
    let str = String::from("R            ");
    println!("{}", trimmed_space(&str));
    let str = String::from(" R            ");
    println!("{}", trimmed_space(&str));
}
