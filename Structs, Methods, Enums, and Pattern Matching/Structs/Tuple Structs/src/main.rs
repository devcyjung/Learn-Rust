#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);
#[derive(Debug)]
struct Unit();

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let _unit = Unit();

    println!("Color({}, {}, {})", black.0, black.1, black.2);
    println!("Point({}, {}, {})", origin.0, origin.1, origin.2);

    println!("=================================================");

    println!("{:?}\n{:?}\n{:?}", black, origin, _unit);

    let slice: &[i32] = &[1, 2, 3, 4, 5];
    println!(
        "{} {} {} {} {}",
        slice[0], slice[1], slice[2], slice[3], slice[4]
    );

    let slice: [i32; 5] = [11, 12, 13, 14, 15];
    println!(
        "{} {} {} {} {}",
        slice[0], slice[1], slice[2], slice[3], slice[4]
    );
}
