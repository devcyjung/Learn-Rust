enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move x: {}, y: {}", x, y),
            Message::Write(str) => println!("Write: {}", str),
            Message::ChangeColor(r, g, b) => println!("Change color r: {}, g: {}, b: {}", r, g, b),
        }
    }
}

fn main() {
    let mut m = Message::Write(String::from("hello"));
    m.call();
    m = Message::Move { x: 300, y: 400 };
    m.call();
    m = Message::ChangeColor(0, 160, 200);
    m.call();
    m = Message::Quit;
    m.call();
}
