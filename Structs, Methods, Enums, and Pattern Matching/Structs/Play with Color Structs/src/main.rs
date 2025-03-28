struct ColorClassicStruct {
    name: String,
    hex: String,
}

struct ColorTupleStruct(String, String);

fn classic_c_structs() -> ColorClassicStruct {
    ColorClassicStruct {
        name: "green".to_string(),
        hex: "#00FF00".to_string(),
    }
}

fn tuple_structs() -> ColorTupleStruct {
    let c_struct = classic_c_structs();
    ColorTupleStruct(c_struct.name, c_struct.hex)
}

fn main() {
    let cl_str = classic_c_structs();
    println!("Classic Struct:");
    println!("Name: {}", cl_str.name);
    println!("Hex: {}", cl_str.hex);

    let tup_str = tuple_structs();
    println!("Tuple Struct:");
    println!("Name: {}", tup_str.0);
    println!("Hex: {}", tup_str.1);
}
