struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.1, y: 4.2 };
    let integer_and_float = Point { x: 5, y: 4.5 };
    let z1 = both_integer.x + both_integer.y;
    let z2 = both_float.x + both_float.y;
    let z3 = integer_and_float.x as f64 + integer_and_float.y;
    println!("z1 = {}", z1);
    println!("z2 = {:.1}", z2);
    println!("z3 = {}", z3);
}
