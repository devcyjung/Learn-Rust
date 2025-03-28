use std::collections::*;

fn main() {
    // put you code here to launch it
    let mut hashmap = HashMap::new();
    hashmap.insert(1, 2);
    hashmap.insert(22, 3);

    for (key, value) in hashmap {
        println!("{} : {}", key, value);
    }
}
