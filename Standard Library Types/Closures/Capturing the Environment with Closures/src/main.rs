#![feature(linked_list_remove, test)]

extern crate test;

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IllegalStateError {
    message: String,
}
impl fmt::Display for IllegalStateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Illegal State Error: {}", self.message)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
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

impl<const N: usize> From<[ShirtColor; N]> for Inventory {
    fn from(shirts: [ShirtColor; N]) -> Self {
        Self::from(&shirts[..])
    }
}

impl<const N: usize> From<&[ShirtColor; N]> for Inventory {
    fn from(shirts: &[ShirtColor; N]) -> Self {
        Self::from(&shirts[..])
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
                Err(_) => {
                    let most_available = self.most_stocked();
                    match most_available {
                        None => Err(IllegalStateError {
                            message: String::from("No shirts in stock"),
                        }),
                        Some(most_available_color) => {
                            let try_giveaway = self.remove_stock(most_available_color);
                            match try_giveaway {
                                Err(_) => Err(IllegalStateError {
                                    message: String::from("No shirts in stock"),
                                }),
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
        let res = match stock {
            None => Err(IllegalStateError {
                message: format!("No such shirt: {:?}", shirt),
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
        };
        if let Some(0) = self.stock.get(&shirt) {
            self.stock.remove(&shirt);
        }
        res
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
    const INVENTORY_SIZE: usize = 30;
    const USER_COUNT: usize = 40;

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::Entry::Occupied;
    use test::Bencher;

    fn setup<'a>(
        inventory_size: usize,
        user_population: usize,
    ) -> (Inventory, Vec<&'a Option<ShirtColor>>) {
        let mut store = Inventory::from(vec![]);
        let mut test_rng = rand::rng();
        Choose::new(&ITEMS)
            .unwrap()
            .sample_iter(&mut test_rng)
            .take(inventory_size)
            .for_each(|&item| store.add_stock(item));
        let pref_vec: Vec<&Option<ShirtColor>> = Choose::new(&PREFERENCES)
            .unwrap()
            .sample_iter(&mut test_rng)
            .take(user_population)
            .collect();
        (store, pref_vec)
    }

    #[test]
    fn shirtcolor_equality_test() {
        assert_eq!(
            vec![ShirtColor::Black, ShirtColor::White],
            vec![ShirtColor::Black, ShirtColor::White],
            "Vec<ShirtColor> fails structural equality test"
        );

        assert_eq!(
            &[ShirtColor::Black, ShirtColor::Green],
            &[ShirtColor::Black, ShirtColor::Green],
            "&[ShirtColor;2] fails structural equality test"
        );

        assert_eq!(
            &[ShirtColor::Black, ShirtColor::Red][..],
            &[ShirtColor::Black, ShirtColor::Red][..],
            "&[ShirtColor] fails structural equality test"
        );

        assert_ne!(
            vec![ShirtColor::Black, ShirtColor::White],
            vec![ShirtColor::White, ShirtColor::Black],
            "Vec<ShirtColor> fails structural inequality test"
        );

        assert_eq!(
            HashMap::from([(ShirtColor::Black, 32), (ShirtColor::White, 21)]),
            HashMap::from([(ShirtColor::White, 21), (ShirtColor::Black, 32)]),
            "HashMap<ShirtColor, i32> fails structural equality test"
        );

        assert_eq!(
            LinkedList::from([ShirtColor::White, ShirtColor::Black]),
            LinkedList::from([ShirtColor::White, ShirtColor::Black]),
            "LinkedList<ShirtColor> fails structural equality test"
        );

        assert_ne!(
            LinkedList::from([ShirtColor::White, ShirtColor::Black]),
            LinkedList::from([ShirtColor::Black, ShirtColor::White]),
            "LinkedList<ShirtColor> fails structural inequality test"
        );
    }

    #[test]
    fn inventory_constructor_test() {
        assert_eq!(
            Inventory::from([ShirtColor::Black, ShirtColor::White]),
            Inventory::from(vec![ShirtColor::Black, ShirtColor::White]),
            "Inventory::from([ShirtColor;N]) != Inventory::from(Vec<ShirtColor>)"
        );

        assert_eq!(
            Inventory::from(&[ShirtColor::White, ShirtColor::Black][..]),
            Inventory::from(vec![ShirtColor::White, ShirtColor::Black]),
            "Inventory::from(&[ShirtColor]) != Inventory::from(Vec<ShirtColor>)"
        );

        assert_eq!(
            Inventory::from(&[ShirtColor::Red, ShirtColor::Yellow][..]),
            Inventory::from(&[ShirtColor::Red, ShirtColor::Yellow]),
            "Inventory::from(&[ShirtColor]) != Inventory::from(&[ShirtColor;N])"
        );

        assert_ne!(
            Inventory::from(&[ShirtColor::Yellow, ShirtColor::Red]),
            Inventory::from(&[ShirtColor::Red, ShirtColor::Yellow]),
            "Inventory from different order of items should be not equal"
        );
    }

    #[test]
    fn inventory_add_stock_test() {
        let mut store = Inventory::from([ShirtColor::Red, ShirtColor::Green]);
        for item in [
            ShirtColor::Blue,
            ShirtColor::Yellow,
            ShirtColor::Black,
            ShirtColor::Red,
            ShirtColor::White,
            ShirtColor::Yellow,
        ] {
            store.add_stock(item)
        }
        let answer_array = [
            ShirtColor::Red,
            ShirtColor::Green,
            ShirtColor::Blue,
            ShirtColor::Yellow,
            ShirtColor::Black,
            ShirtColor::Red,
            ShirtColor::White,
            ShirtColor::Yellow,
        ];
        let answer_map = HashMap::from([
            (ShirtColor::Red, 2),
            (ShirtColor::Green, 1),
            (ShirtColor::Blue, 1),
            (ShirtColor::Yellow, 2),
            (ShirtColor::Black, 1),
            (ShirtColor::White, 1),
        ]);
        let inventory_from = Inventory::from(answer_array);
        let shirt_list_answer = LinkedList::from(answer_array);
        let inventory_answer = Inventory {
            stock: answer_map.clone(),
            shirt_list: shirt_list_answer.clone(),
        };
        assert_eq!(
            store.shirt_list, shirt_list_answer,
            "Wrong store.shirt_list",
        );
        assert_eq!(store.stock, answer_map, "Wrong store.stock",);
        assert_eq!(store, inventory_answer, "Wrong store",);
        assert_eq!(
            store, inventory_from,
            "add_stock result != Inventory::from result",
        );
    }

    #[test]
    fn inventory_remove_stock_test() {
        let mut store1 = Inventory::from([
            ShirtColor::Red,
            ShirtColor::Green,
            ShirtColor::Black,
            ShirtColor::Red,
            ShirtColor::White,
            ShirtColor::Black,
        ]);
        let mut store2 = store1.clone();
        let mut store3 = store1.clone();
        let mut store4 = Inventory::from([]);
        let store1_err_blue = store1.remove_stock(ShirtColor::Blue);
        let store1_ok_1_red = store1.remove_stock(ShirtColor::Red);
        let store1_ok_1_black = store1.remove_stock(ShirtColor::Black);
        let store2_ok_0_green = store2.remove_stock(ShirtColor::Green);
        let store3_ok_1_black = store3.remove_stock(ShirtColor::Black);
        let store3_err_yellow = store3.remove_stock(ShirtColor::Yellow);
        let store4_err_red = store4.remove_stock(ShirtColor::Red);

        assert_eq!(
            store1_err_blue,
            Err(IllegalStateError {
                message: format!("No such shirt: {:?}", ShirtColor::Blue)
            }),
            "Wrong store1_err"
        );
        assert_eq!(store1_ok_1_red, Ok(1u128), "Wrong store1_ok_5");
        assert_eq!(store1_ok_1_black, Ok(1u128), "Wrong store1_ok_4");
        assert_eq!(store2_ok_0_green, Ok(0u128), "Wrong store2_ok_5");
        assert_eq!(store3_ok_1_black, Ok(1u128), "Wrong store3_ok_5");
        assert_eq!(
            store3_err_yellow,
            Err(IllegalStateError {
                message: format!("No such shirt: {:?}", ShirtColor::Yellow)
            }),
            "Wrong store3_err"
        );
        assert_eq!(
            store4_err_red,
            Err(IllegalStateError {
                message: format!("No such shirt: {:?}", ShirtColor::Red)
            }),
            "Wrong store4_err"
        );

        let store1_answer = Inventory::from([
            ShirtColor::Green,
            ShirtColor::Red,
            ShirtColor::White,
            ShirtColor::Black,
        ]);
        let store2_answer = Inventory::from([
            ShirtColor::Red,
            ShirtColor::Black,
            ShirtColor::Red,
            ShirtColor::White,
            ShirtColor::Black,
        ]);
        let store3_answer = Inventory::from([
            ShirtColor::Red,
            ShirtColor::Green,
            ShirtColor::Red,
            ShirtColor::White,
            ShirtColor::Black,
        ]);

        assert_eq!(store1, store1_answer, "Wrong store1");
        assert_eq!(store2, store2_answer, "Wrong store2");
        assert_eq!(store3, store3_answer, "Wrong store3");
        assert_eq!(store4, Inventory::from([]), "Wrong store4");
    }

    #[test]
    fn most_stocked_test() {
        let store1 = Inventory::from([]);
        let store2 = Inventory::from([ShirtColor::Red]);
        let store3 = Inventory::from([ShirtColor::Green, ShirtColor::Red]);
        let store4 = Inventory::from([ShirtColor::White, ShirtColor::Blue, ShirtColor::White]);
        assert_eq!(
            store1.most_stocked(),
            None,
            "Wrong most_stocked() for empty Inventory"
        );
        assert_eq!(
            store2.most_stocked(),
            Some(ShirtColor::Red),
            "Wrong most_stocked() for single item Inventory"
        );
        assert!(
            store3.most_stocked() == Some(ShirtColor::Green)
                || store3.most_stocked() == Some(ShirtColor::Red),
            "Wrong most_stocked() for Inventory with tied most stocked items: {:?}",
            store3.most_stocked()
        );
        assert_eq!(
            store4.most_stocked(),
            Some(ShirtColor::White),
            "Wrong most_stocked() for multiple items Inventory"
        );
        for (i, s) in [store1, store2, store3, store4].iter().enumerate() {
            assert_eq!(
                s.most_stocked(),
                s.most_stocked(),
                "most_stocked() not idempotent for: store{}",
                i + 1
            )
        }
    }

    #[test]
    fn giveaway_test() {
        let mut store1 = Inventory::from([
            ShirtColor::Red,
            ShirtColor::Blue,
            ShirtColor::Red,
            ShirtColor::White,
        ]);
        let store1_give_red = store1.giveaway(None);

        assert_eq!(
            store1,
            Inventory::from([ShirtColor::Blue, ShirtColor::Red, ShirtColor::White,]),
            "Wrong giveaway for store1"
        );
        assert_eq!(
            store1_give_red,
            Ok(ShirtColor::Red),
            "Wrong giveaway in store1 for red preference"
        );

        let mut store2 = Inventory::from([]);
        let store2_give_err_red = store2.giveaway(Some(ShirtColor::Red));
        let store2_give_err_none = store2.giveaway(None);

        let empty_stock_error = Err(IllegalStateError {
            message: String::from("No shirts in stock"),
        });

        assert_eq!(store2, Inventory::from([]), "Wrong giveaway for store2");
        assert_eq!(
            store2_give_err_red, empty_stock_error,
            "Wrong giveaway in store2 for red preference"
        );
        assert_eq!(
            store2_give_err_none, empty_stock_error,
            "Wrong giveaway in store2 for none preference"
        );

        let mut store3 = Inventory::from([
            ShirtColor::Red,
            ShirtColor::Green,
            ShirtColor::Blue,
            ShirtColor::Red,
        ]);
        let store3_give_red_1 = store3.giveaway(Some(ShirtColor::Black));
        let store3_give_red_2 = store3.giveaway(Some(ShirtColor::Red));
        let store3_give_blue = store3.giveaway(Some(ShirtColor::Blue));
        let store3_give_green = store3.giveaway(Some(ShirtColor::Yellow));
        let store3_give_err_none = store3.giveaway(None);
        let store3_give_err_red = store3.giveaway(Some(ShirtColor::Red));

        assert_eq!(store3, Inventory::from([]), "Wrong giveaway for store3");
        assert_eq!(
            store3_give_red_1,
            Ok(ShirtColor::Red),
            "Wrong giveaway in store3 for black preference when black is out of stock"
        );
        assert_eq!(
            store3_give_red_2,
            Ok(ShirtColor::Red),
            "Wrong giveaway in store3 for red preference"
        );
        assert_eq!(
            store3_give_blue,
            Ok(ShirtColor::Blue),
            "Wrong giveaway in store3 for blue preference"
        );
        assert_eq!(
            store3_give_green,
            Ok(ShirtColor::Green),
            "Wrong giveaway in store3 for yellow preference when yellow is out of stock"
        );
        assert_eq!(
            store3_give_err_none, empty_stock_error,
            "Wrong giveaway in store3 for none preference when stock is empty"
        );
        assert_eq!(
            store3_give_err_red, empty_stock_error,
            "Wrong giveaway in store3 for red preference when stock is empty"
        );
    }

    #[test]
    fn inventory_fuzz_test() {
        let empty_stock_error = Err(IllegalStateError {
            message: String::from("No shirts in stock"),
        });
        let (mut store1, pref_vec1) = setup(17, 20);
        for (i, &&p1) in pref_vec1.iter().enumerate() {
            let pref_color_is_in_stock = if let Some(pref) = p1 {
                Some(matches!(store1.stock.entry(pref), Occupied(_)))
            } else {
                None
            };
            let most_in_stock = store1.most_stocked();
            let res = store1.giveaway(p1);
            if i < 17 {
                match pref_color_is_in_stock {
                    None => assert_eq!(
                        res,
                        Ok(most_in_stock.unwrap()),
                        "Should give most abundant shirt to none preference
                        {:#?}
                        {:?}",
                        store1,
                        p1
                    ),
                    Some(true) => assert_eq!(
                        res,
                        Ok(p1.unwrap()),
                        "Should give preferred shirt\n{:#?}\n{:?}",
                        store1,
                        p1
                    ),
                    Some(false) => assert_eq!(
                        res,
                        Ok(most_in_stock.unwrap()),
                        "Should give most abundant shirt when preferred shirt is out of stock
                        {:#?}
                        {:?}",
                        store1,
                        p1
                    ),
                }
            } else {
                assert_eq!(
                    res, empty_stock_error,
                    "Stock should be empty\n{:#?}\n{:?}",
                    store1, p1
                );
            }
        }

        let (mut store2, pref_vec2) = setup(20, 17);
        for (i, &&p2) in pref_vec2.iter().enumerate() {
            let pref_color_is_in_stock = if let Some(pref) = p2 {
                Some(matches!(store2.stock.entry(pref), Occupied(_)))
            } else {
                None
            };
            let most_in_stock = store2.most_stocked();
            let res = store2.giveaway(p2);
            if pref_color_is_in_stock.is_none() {
                assert_eq!(
                    res,
                    Ok(most_in_stock.unwrap()),
                    "Should give most abundant shirt to none preference\n{:#?}\n{:?}",
                    store2,
                    p2
                )
            } else if let Some(true) = pref_color_is_in_stock {
                assert_eq!(
                    res,
                    Ok(p2.unwrap()),
                    "Should give preferred shirt
                    {:#?}
                    {:?}",
                    store2,
                    p2
                )
            } else {
                assert_eq!(
                    res,
                    Ok(most_in_stock.unwrap()),
                    "Should give most abundant shirt when preferred shirt is out of stock
                    {:#?}
                    {:?}",
                    store2,
                    p2
                )
            }
        }
    }

    #[bench]
    fn giveaway_bench(b: &mut Bencher) {
        let (mut store, prefs) = setup(10000000, 1200000);
        b.iter(|| {
            for &&p in &prefs {
                let _ = store.giveaway(p);
            }
        })
    }
}
