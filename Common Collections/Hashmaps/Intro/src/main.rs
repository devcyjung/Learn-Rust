use std::collections::HashMap;

fn main() {
    let scores = HashMap::<&str, u32>::from([
        ("Arsenal", 52),
        ("Manchester utd.", 36),
        ("Real Madrid", 72),
        ("Paris Saint Germain", 32),
        ("FC Barcelona", 52),
    ]);

    println!("{:>20}{:>9}", "Team", "Score");

    for (name, score) in scores {
        println!("{:>20}{:>9}", name, score);
    }
}
