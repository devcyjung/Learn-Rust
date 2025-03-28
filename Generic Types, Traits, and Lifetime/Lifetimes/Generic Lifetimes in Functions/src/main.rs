fn main() {
    let string1 = String::from("abcd");
    let string2 = "abcd";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn longest<'a, 'b, 'c>(x: &'a str, y: &'b str) -> &'c str
where
    'a: 'c, // 'a outlives 'c
    'b: 'c, // 'b outlives 'c
{
    let s = match (x.len(), y.len()) {
        (a, b) if a >= b => x,
        (a, b) if a < b => y,
        (a, b) => panic!(
            "x.len() and y.len() are not comparable for some reason. {} {} {} {}",
            a, b, x, y
        ),
    };
    s
}
