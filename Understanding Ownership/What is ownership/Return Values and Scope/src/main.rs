fn main() {
    let mut s1 = gives_ownership(); // gives_ownership moves its return
                                    // value into s1
    s1.push_str(" world");

    let s2 = s1.clone(); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3
    println!("s1={}, s3={}", s1, s3)
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    String::from("hello") // some_string comes into scope

    // some_string is returned and
    // moves out to the calling
    // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(mut a_string: String) -> String {
    // a_string comes into
    // scope
    a_string.push_str(" some added stuff");
    a_string // a_string is returned and moves out to the calling function
}
