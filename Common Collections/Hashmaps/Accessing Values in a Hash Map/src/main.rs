use rand::Rng;
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    let mut rng = rand::rng();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.insert("Green".to_string(), 11);
    scores.insert("Purple".to_string(), 12);
    let option1 = scores.insert("Black".to_string(), 9);
    let option2 = scores.insert(String::from("Purple"), -4);

    match option1 {
        Some(value) => println!("The value of option1 is: {}", value),
        None => println!("Option1 is None"),
    }

    match option2 {
        Some(value) => println!("The value in option2 is {}", value),
        None => println!("Option2 is None"),
    }

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    println!("{} team's score is {}\n", team_name, score.unwrap());

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    println!();

    for value in scores.values_mut() {
        match value {
            ..11 => *value = rng.random_range(10..20),
            11.. => *value = rng.random_range(200..250),
        }
    }

    for (key, value) in scores.iter() {
        println!("{}: {}", key, value);
    }

    println!();

    for value in scores.values_mut() {
        match value {
            num if num == &17 => *num = 20,
            num if num == &18 => *num = 30,
            num if num > &mut 16 => *num = 40,
            _ => *value = 50,
        }
    }

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}
