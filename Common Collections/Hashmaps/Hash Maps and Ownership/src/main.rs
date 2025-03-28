use std::collections::HashMap;

fn main() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them
    // and see what compiler error you get!
    //println!("{}, {}", field_name, field_value);
    println!("{:#?}", map);

    let mut map = HashMap::new();
    map.insert("Some key", "Some value");
    let k = "Another key";
    let v = "Another value";
    map.insert(k, v);
    println!("{} {}", k, v);
    println!("{:#?}", map);
}
