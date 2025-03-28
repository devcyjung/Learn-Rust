fn main() {
    let _empty_string = String::new();
    print(&_empty_string);
    let data = "initial contents";
    print(data);
    let s = data.to_string();
    print(&s);
    // the method also works on a literal directly:
    let s = "initial contents".to_string();
    print(&s);
    let hello = String::from("السلام عليكم");
    print(&hello);
    let hello = String::from("Dobrý den");
    print(&hello);
    let hello = String::from("Hello");
    print(&hello);
    let hello = String::from("שָׁלוֹם");
    print(&hello);
    let hello = String::from("नमस्ते");
    print(&hello);
    let hello = String::from("こんにちは");
    print(&hello);
    let hello = String::from("안녕하세요");
    print(&hello);
    let hello = String::from("你好");
    print(&hello);
    let hello = String::from("Olá");
    print(&hello);
    let hello = String::from("Здравствуйте");
    print(&hello);
    let hello = String::from("Hola");
    print(&hello);
}

fn print(s: &str) {
    println!("{}", s);
}
