use rand::Rng;
use std::collections::HashMap;

fn main() {
    let secret_number1 = rand::thread_rng().gen_range(1..=100);
    let secret_number2 = rand::thread_rng().gen_range(1..=100);
    let secret_number3 = rand::thread_rng().gen_range(1..=100);
    let hashmap = HashMap::from([
        ("James", secret_number1),
        ("Jeremy", secret_number2),
        ("Richard", secret_number3),
    ]);

    for (presenter, value) in hashmap {
        println!("{} : {}", presenter, value);
    }
}
