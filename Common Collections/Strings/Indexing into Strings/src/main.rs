fn main() {
    let s1 = String::from("안녕하세요 مرحبا Здравствуйте 你好 こんにちは");
    // ERROR!!! String doesn't support indexing!
    // let h = s1[0];

    let h = s1.chars().take(1).last().unwrap();
    println!("s1 is {}", s1);
    println!("h is {}", h);

    for c in s1.chars() {
        println!("c is {}", c);
    }

    for b in s1.bytes() {
        println!("b is {}", b);
    }
}
