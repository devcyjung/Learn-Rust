fn main() {
    let x = 5;

    let y = {
        let x = 3;
        // No-Semicolon: Expression block - Returns a value
        // Semicolon: Statement block - Does not return a value
        x + 1
    };

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
