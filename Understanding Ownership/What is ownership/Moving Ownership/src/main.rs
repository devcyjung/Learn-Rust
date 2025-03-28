fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    // !!! ERROR: value is moved to s2
    // println!("{}, world!", s1);

    let v1 = String::from("hello");
    let v2 = v1.clone();

    println!("{}, world!", s2);
    println!("{}, world!", v1);
    println!("{}, world!", v2);
}
