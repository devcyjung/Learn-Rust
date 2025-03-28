fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        static STRING2: &str = "xyz";
        // !!! ERROR: `string2` does not live long enough
        result = longest(string1.as_str(), STRING2);
    }
    println!("The longest string is {}", result);
}
