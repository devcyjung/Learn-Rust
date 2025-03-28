pub fn trim_me(input: &str) -> String {
    input.trim().to_string()
}

pub fn compose_me(input: &str) -> String {
    let add = " world!";
    let s1 = Into::<String>::into(input) + add;
    let s2 = input.to_owned() + add;
    let s3 = format!("{}{}", input, add);
    let s4 = String::from(input) + add;
    let s5 = input.to_string() + add;
    assert_eq!(s1, s2);
    assert_eq!(s2, s3);
    assert_eq!(s3, s4);
    assert_eq!(s4, s5);
    s3
}

pub fn replace_me(input: &str) -> String {
    input.replace("cars", "balloons")
}
