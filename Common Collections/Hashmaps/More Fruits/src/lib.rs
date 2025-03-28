use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq, Debug)]
pub enum Fruit {
    Apple,
    Banana,
    Mango,
    Lichi,
    Pineapple,
}

pub fn fruit_basket(basket: &mut HashMap<Fruit, u32>) {
    let fruit_kinds = vec![
        Fruit::Apple,
        Fruit::Banana,
        Fruit::Mango,
        Fruit::Lichi,
        Fruit::Pineapple,
    ];

    for fruit in fruit_kinds {
        match fruit {
            e if e == Fruit::Apple || e == Fruit::Mango || e == Fruit::Lichi => continue,
            _ => {
                let cur = basket.entry(fruit).or_insert(0);
                *cur += 1;
            }
        }
    }

    println!("{:?}", basket);
}
