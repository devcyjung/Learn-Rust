fn main() {
    let v1 = [1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {val}");
    }

    for val in v1 {
        println!("Got: {val}");
    }

    // Value moved in for loop
    // println!("{:?}", v1_iter.next());
}
