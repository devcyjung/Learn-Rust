fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    // let first = &v[0];

    // v.push(6); // ERROR!!!

    // println!("The first element is: {}", first);

    v.push(6);
    let first = &v[0];
    println!("{:?} {}", v, first);
}
