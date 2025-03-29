#![feature(linked_list_remove)]

/**
4 Different for loops over vec & slice
I) for shirt in shirts
I-1) shirts: Vec<ShirtColor>
for shirt: ShirtColor in shirts (moves shirts)
I-2) shirts: &[ShirtColor]
for shirt: &ShirtColor in shirts (borrows)

II) for &shirt in shirts
II-1) shirts: Vec<ShirtColor> -> Invalid (expected ShirtColor but found &_)
II-2) shirts: &[ShirtColor]
for &shirt: ShirtColor in shirts (borrows)

III) for shirt in &shirts
III-1) shirts: Vec<ShirtColor>
for shirt: &ShirtColor in &shirts (borrows)
III-2) shirts: &[ShirtColor] -> Invalid (&&[ShirtColor] is not iterator)

IV) for &shirt in &shirts
IV-1) shirts: Vec<ShirtColor>
for &shirt: ShirtColor in &shirts (borrows)
IV-2) shirts: &[ShirtColor] -> Invalid (&&[ShirtColor] is not iterator)
*/
use std::{
    collections::{hash_map, HashMap, LinkedList},
    fmt,
};


use rand::{
    self,
    distr::{slice::Choose, Distribution},
};

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum ShirtColor {
    Red,
    Blue,
    Green,
    Yellow,
    Black,
    White,
}

#[derive(Debug, Clone)]
pub struct IllegalStateError {
    message: String,
}
impl fmt::Display for IllegalStateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Illegal State Error: {}", self.message)
    }
}

#[derive(Debug)]
pub struct Inventory {
    pub stock: HashMap<ShirtColor, u128>,
    pub shirt_list: LinkedList<ShirtColor>,
}

impl From<&[ShirtColor]> for Inventory {
    fn from(shirts: &[ShirtColor]) -> Self {
        let mut stock = HashMap::new();
        let mut shirt_list = LinkedList::new();
        for &shirt in shirts {
            *stock.entry(shirt).or_insert(0) += 1;
            shirt_list.push_back(shirt);
        }
        Self { stock, shirt_list }
    }
}

impl From<Vec<ShirtColor>> for Inventory {
    fn from(shirts: Vec<ShirtColor>) -> Self {
        Self::from(shirts.as_slice())
    }
}

impl Inventory {
    pub fn giveaway(
        &mut self,
        user_preference: Option<ShirtColor>,
    ) -> Result<ShirtColor, IllegalStateError> {
        let shirt = user_preference.or_else(|| self.most_stocked());
        shirt.map_or_else(
            || {
                Err(IllegalStateError {
                    message: String::from("No shirts in stock"),
                })
            },
            |shirt_to_give| match self.remove_stock(shirt_to_give) {
                Err(shirt_to_give_not_found_error) => {
                    let most_available = self.most_stocked();
                    match most_available {
                        None => Err(IllegalStateError {
                            message: String::from("No shirts in stock"),
                        }),
                        Some(most_available_color) => {
                            let try_giveaway = self.remove_stock(most_available_color);
                            match try_giveaway {
                                Err(_) => Err(shirt_to_give_not_found_error),
                                Ok(_) => Ok(most_available_color),
                            }
                        }
                    }
                }
                Ok(_) => Ok(shirt_to_give),
            },
        )
    }

    pub fn add_stock(&mut self, shirt: ShirtColor) {
        *self.stock.entry(shirt).or_insert(0) += 1;
        self.shirt_list.push_back(shirt);
    }

    pub fn remove_stock(&mut self, shirt: ShirtColor) -> Result<u128, IllegalStateError> {
        let stock = match self.stock.entry(shirt) {
            hash_map::Entry::Occupied(o) => Some(o.into_mut()),
            hash_map::Entry::Vacant(_) => None,
        };
        match stock {
            None => Err(IllegalStateError {
                message: format!("No such shirt {:?}", shirt),
            }),
            Some(stock) if *stock == 0 => Err(IllegalStateError {
                message: format!("Cannot remove stock: shirt {:?} stock {}", shirt, stock),
            }),
            Some(stock) => match self.shirt_list.iter().position(|&l| l == shirt) {
                None => Err(IllegalStateError {
                    message: format!(
                        "Hash map and LinkedList are incoherent.\n
                        Hashmap: {:?}, LinkedList: {:?}",
                        self.stock, self.shirt_list
                    ),
                }),
                Some(item_index) => {
                    self.shirt_list.remove(item_index);
                    *stock -= 1;
                    Ok(*stock)
                }
            },
        }
    }

    pub fn most_stocked(&self) -> Option<ShirtColor> {
        let (mut max_item, mut max_count) = (None, 0u128);
        for (item, stock) in &self.stock {
            if max_count < *stock {
                max_count = *stock;
                max_item = Some(*item);
            }
        }
        max_item
    }
}

fn main() {
    const ITEMS: [ShirtColor; 6] = [
        ShirtColor::Red,
        ShirtColor::Blue,
        ShirtColor::Green,
        ShirtColor::Yellow,
        ShirtColor::Black,
        ShirtColor::White,
    ];
    const PREFERENCES: [Option<ShirtColor>; 7] = [
        Some(ShirtColor::Red),
        Some(ShirtColor::Blue),
        Some(ShirtColor::Green),
        Some(ShirtColor::Yellow),
        Some(ShirtColor::Black),
        Some(ShirtColor::White),
        None,
    ];
    const INVENTORY_SIZE: usize = 10;
    const USER_COUNT: usize = 12;

    let mut store = Inventory::from(vec![]);
    let mut local_rng = rand::rng();
    let items_dist = Choose::new(&ITEMS).unwrap();
    items_dist
        .sample_iter(&mut local_rng)
        .take(INVENTORY_SIZE)
        .for_each(|&item| store.add_stock(item));

    println!("Initial Store Status: {:?}", store);

    let prefs_dist = Choose::new(&PREFERENCES).unwrap();
    prefs_dist
        .sample_iter(&mut local_rng)
        .take(USER_COUNT)
        .for_each(|&pref| {
            let g = store.giveaway(pref);
            println!(
                "Pref: {:?}\nGiveaway Result: {:?}\nStore Status:{:?}",
                pref, g, store
            );
        });
}
