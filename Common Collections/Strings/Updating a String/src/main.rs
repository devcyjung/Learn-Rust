fn main() {
    // Appending a string slice to a String using the push_str method
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("s is {}", s);

    // Using a string slice after appending its contents to a String
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1 is {}, s2 is {}", s1, s2);

    // Adding one character to a String value using push
    let mut s = String::from("lo");
    s.push('l');
    println!("s is {}", s);

    // Using the + operator to combine two String values into a new String value
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("s2 is {}, s3 is {}", s2, s3);

    // Using format! macro
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s is {} s1 is {} s2 is {} s3 is {}", s, s1, s2, s3);
}
