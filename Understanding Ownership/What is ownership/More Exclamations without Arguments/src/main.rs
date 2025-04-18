fn main() {
    let mut hello1 = with_exclamation();

    println!("{} is `{}`", "hello1", hello1);

    hello1.push_str("!");

    println!("{} is `{}`", "hello1", hello1);
}

fn with_exclamation() -> String {
    /* Create the "Hello" string here */
    let mut hello = String::from("Hello");
    hello.push_str("!");
    hello
}
