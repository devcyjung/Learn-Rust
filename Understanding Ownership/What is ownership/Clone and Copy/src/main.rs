fn main() {
    let s1: String = String::from("hello");
    let s2: String = s1.clone(); // Clone

    println!("s1 = {}, s2 = {}", s1, s2);

    let x: i32 = 5;
    let y: i32 = x; // Copy

    println!("x = {}, y = {}", x, y);
}
