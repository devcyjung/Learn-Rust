fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?} {:?} {:?}", five, six, none);
}

fn plus_one(maybe_int: Option<i32>) -> Option<i64> {
    maybe_int.map(|i| -> i64 { i as i64 + 1 })
}
