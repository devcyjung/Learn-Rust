use std::collections::hash_map::Entry;
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    // 1. Discard old value, Take new value
    // insert -> Option<V> (Some<V> / None)
    let o1 = scores.insert(String::from("Blue"), 10);
    let o2 = scores.insert(String::from("Blue"), 25);
    println!("scores {:?}\no1 {:?}\no2 {:?}", scores, o1, o2);

    // 2. Retain old value, Take new value if absent
    // entry ->
    // Entry<K, V> (Entry::Occupied(OccupiedEntry<K, V>) / Entry::Vacant(VacantEntry<K, V>))
    // entry.or_insert -> &mut V
    let yellow = scores.entry(String::from("Yellow"));
    println!("yellow {:?}", yellow);

    match yellow {
        Entry::Occupied(o) => {
            println!("Occupied Yellow {} : {}", o.key(), o.get());
            let clone_key = o.key().clone();
            let mut_o = o.into_mut();
            *mut_o = 35;
            println!("Occupied Yellow Altered {} : {}", clone_key, mut_o);
        }
        Entry::Vacant(v) => {
            println!("Vacant Yellow {}", v.key());
            v.insert(69);
            println!("Vacant Yellow {:?}", scores);
        }
    }

    println!("After yellow match {:?}", scores);

    let blue = scores.entry(String::from("Blue"));
    println!("blue {:?}", blue);

    match blue {
        Entry::Occupied(o) => {
            println!("Occupied Blue {} : {}", o.key(), o.get());
            let clone_key = o.key().clone();
            let mut_o = o.into_mut();
            *mut_o = 42;
            println!("Occupied Blue Altered {} : {}", clone_key, mut_o);
        }
        Entry::Vacant(v) => {
            println!("Vacant Blue {}", v.key());
            v.insert(420);
            println!("Vacant Blue {:?}", scores);
        }
    }

    println!("After blue match {:?}", scores);

    let oi1 = scores.entry(String::from("Red")).or_insert(50);
    println!("oi1 content {}", *oi1);

    let oi2 = scores.entry(String::from("Blue")).or_insert(77);
    println!("oi2 content {}", *oi2);
    println!("After or insert {:?}", scores);

    // 3. Update Based on Old Value (Insert if absent, update based on old value if present)
    // mut_r = entry.or_insert
    // *mut_r = f(*mut_r)
    let text = "hello world wonderful world abc abc abc";
    let mut map = HashMap::<&str, f64>::new();
    for word in text.split_whitespace() {
        let not_count = map.entry(word).or_insert(1f64);
        *not_count = *not_count * 2f64 / word.len() as f64;
    }
    println!("{:?} {}", map, 8f64 / 27f64);
}
