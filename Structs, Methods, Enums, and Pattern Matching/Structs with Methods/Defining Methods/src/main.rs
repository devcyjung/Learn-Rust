#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u64 {
        self.width as u64 * self.height as u64
    }
}

fn main() {
    let rectangle = Rectangle {
        width: 0xFFFFFFFF,
        height: 0xFFFFFFFF,
    };
    println!("The area of the rectangle is {}", rectangle.area());

    let point = Point { x: 0, y: 0 };
    println!("{}", point.x());
    let point = Point {
        x: "d1",
        y: "hello",
    };
    println!("{}", point.x());
    let point = Point {
        x: "1d".to_string(),
        y: "hello".to_string(),
    };
    println!("{}", point.x());
    let point = Point {
        x: 12..22,
        y: 100..122,
    };
    println!("{:?}", point.x());
}
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
