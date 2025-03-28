extern crate num;
#[macro_use]
extern crate num_derive;

use rand::Rng;
use std::cmp::min;
use std::cmp::PartialEq;
use std::io::stdin;
use std::io::Write;
use std::ops::Add;
use std::thread::sleep;
use std::time::Duration;

struct Player<'a> {
    has_fancy_hat: bool,
    fancy_hat: u32,
    position: Position,
    name: &'a str,
}

#[derive(FromPrimitive, ToPrimitive, Debug)]
enum Position {
    Origin = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Eleven = 11,
    Twelve = 12,
    Thirteen = 13,
    Fourteen = 14,
    Fifteen = 15,
    Sixteen = 16,
    Seventeen = 17,
    Eighteen = 18,
    Nineteen = 19,
    Twenty = 20,
    Goal = 21,
}

impl Clone for Position {
    fn clone(&self) -> Self {
        num::FromPrimitive::from_u8(num::ToPrimitive::to_u8(self).unwrap()).unwrap()
    }
}

impl Add<u8> for Position {
    type Output = Self;

    fn add(self, roll: u8) -> Self {
        let pos: u8 = min(21u8, num::ToPrimitive::to_u8(&self).unwrap() + roll);
        num::FromPrimitive::from_u8(pos).unwrap()
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        num::ToPrimitive::to_u8(self) == num::ToPrimitive::to_u8(other)
    }
}

impl<'a> Player<'a> {
    fn new(name: &'a str) -> Player<'a> {
        Player {
            has_fancy_hat: false,
            fancy_hat: 0,
            position: Position::Origin,
            name,
        }
    }

    fn add_fancy_hat(&mut self) {
        self.has_fancy_hat = true;
        self.fancy_hat += 1;
        println!("Player gained a fancy hat.");
    }

    fn remove_fancy_hat(&mut self) {
        if !self.has_fancy_hat {
        } else if self.fancy_hat == 0 {
            panic!(
                "player.has_fancy_hat() && player.fancy_hat == {}",
                self.fancy_hat
            );
        } else {
            self.fancy_hat -= 1;
            if self.fancy_hat == 0 {
                self.has_fancy_hat = false;
            }
            println!("Player lost a fancy hat.");
        }
    }

    fn move_position(&mut self, dice_roll: u8) {
        self.position = self.position.clone() + dice_roll;
    }

    fn print_status(&self) {
        println!(
            "Player has {} fancy hats, position {:?}",
            self.fancy_hat, self.position
        );
    }
}

fn main() {
    let mut input = String::new();
    prompt_username(&mut input);
    let username = input.clone();
    drop(input);
    let mut player = Player::new(&username);
    game_loop(&mut player);
}

fn prompt_username(str: &mut String) {
    let stdin = stdin();
    loop {
        print!("Enter username (max 64 bytes): ");
        std::io::stdout().flush().unwrap();
        let input = stdin.read_line(str);
        println!();
        match input {
            Ok(_) => {
                *str = str.trim_end().to_string();
                if str.len() >= 64 {
                    println!("Username exceeds limit: max 64 bytes. Try again.");
                } else {
                    println!("Username: {}", str);
                    break;
                }
            }
            Err(err) => {
                println!("Error: {}", err);
            }
        }
    }
}

fn game_loop(player: &mut Player) {
    println!("Game start!");
    while player.position != Position::Goal {
        sleep(Duration::from_secs(1));
        let dice = rand::rng().random_range(1..=6);
        println!("Dice roll: {}", dice);
        match dice {
            2 => player.add_fancy_hat(),
            3 => player.remove_fancy_hat(),
            other => player.move_position(other),
        }
        player.print_status();
    }
    println!("Goal reached, {}!", player.name);
    println!("You have {} fancy hats!", player.fancy_hat);
}
