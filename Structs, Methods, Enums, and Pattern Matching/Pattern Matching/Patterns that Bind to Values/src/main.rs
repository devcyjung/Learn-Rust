#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            match state {
                UsState::Alabama => {
                    println!("Alabaman Quarter");
                }
                UsState::Alaska => {
                    println!("Yep Quarter from Alaska");
                }
            }
            25
        }
    }
}

fn main() {
    let value = value_in_cents(Coin::Quarter(UsState::Alabama));
    println!("{}", value);
    let value = value_in_cents(Coin::Dime);
    println!("{}", value);
    let value = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("{}", value);
    let value = value_in_cents(Coin::Nickel);
    println!("{}", value);
    let value = value_in_cents(Coin::Penny);
    println!("{}", value);
}
