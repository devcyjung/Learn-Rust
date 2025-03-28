#[derive(Debug)]
enum Message {
    Quit,
    ChangeColor(u8, u8, u8),
    Echo(String),
    Move { x: u8, y: u8 },
}

#[derive(Debug)]
struct Point {
    x: u8,
    y: u8,
}

struct State {
    color: (u8, u8, u8),
    position: Point,
    quit: bool,
}

impl State {
    fn change_color(&mut self, color: (u8, u8, u8)) {
        self.color = color;
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn echo(&self, s: String) {
        println!("{}", s);
    }

    fn move_position(&mut self, p: Point) {
        self.position = p;
    }

    fn process(&mut self, message: Message) {
        match message {
            Message::Quit => self.quit(),
            Message::ChangeColor(r, g, b) => self.change_color((r, g, b)),
            Message::Echo(echo) => self.echo(echo),
            Message::Move { x, y } => self.move_position(Point { x, y }),
        }
    }
}

fn test_match_message_call() -> State {
    let mut state = State {
        quit: false,
        position: Point { x: 0, y: 0 },
        color: (0, 0, 0),
    };
    state.process(Message::ChangeColor(255, 0, 255));
    state.process(Message::Echo("hello world".to_string()));
    state.process(Message::Move { x: 10, y: 15 });
    state.process(Message::Quit);

    state
}

fn main() {
    let state = test_match_message_call();
    println!("Color: {:?}", state.color);
    println!("Position: {:?}", state.position);
    if state.quit {
        println!("Quit");
    }
}
