use std::collections::HashMap;

pub fn fruit_basket() -> HashMap<String, u32> {
    let mut basket = HashMap::<String, u32>::new();

    // Two bananas are already given to you :)
    basket.insert(String::from("banana"), 2);
    let vec = [
        "banana",
        "apple",
        "mango",
        "grape",
        "watermelon",
        "strawberries",
        "banana",
        "orange",
    ];

    for &fruit in vec.iter() {
        let cnt = basket.entry(String::from(fruit)).or_insert(0);
        *cnt += 1;
    }

    println!("{:?}", basket);
    basket
}
