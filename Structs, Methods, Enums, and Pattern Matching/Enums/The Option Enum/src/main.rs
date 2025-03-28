fn main() {
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    println!("{:?}", some_number);
    println!("{:?}", some_string);
    println!("{:?}", absent_number);

    let maybe_x: Option<i8> = Some(5);
    let y: i8 = 5;

    // !!! ERROR
    // let sum = x + y;

    let sum = maybe_x.map(|x| -> i16 { x as i16 + y as i16 });
    println!("{:?}", sum);

    let maybe_x: Option<i8> = None;
    let y: i8 = 5;

    let sum = maybe_x.map(|x| -> i16 { x as i16 + y as i16 });
    println!("{:?}", sum);
}
