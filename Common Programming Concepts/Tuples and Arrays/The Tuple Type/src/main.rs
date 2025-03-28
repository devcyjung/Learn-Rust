fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // Pattern matching : destructuring
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
    
    // Indexing: dot + index
    println!("The last element is: {}", tup.2);
}
