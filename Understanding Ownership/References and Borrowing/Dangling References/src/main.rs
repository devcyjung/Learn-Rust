fn main() {
    let reference_to_nothing = dangle();
    println!("{}", reference_to_nothing);
}

// !!! ERROR: returning reference for a local variable
fn dangle() -> String {
    String::from("hello")
}
