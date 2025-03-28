fn main() {
    let s = String::from("hello world");

    let word = first_word(&s);

    // s.clear(); // error!
    println!("the first word of '{}' is: '{}'", s, word);

    let word = second_word(&s);
    println!("the second word of '{}' is: '{}'", s, word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    s
}

fn second_word(s: &str) -> &str {
    let first_word_len = first_word(s).len();

    assert!(first_word_len <= s.len(),
            "In string '{}', its first word's length is computed to be {}, while the whole string length is {}",
            s, first_word_len, s.len());

    if first_word_len + 1 >= s.len() {
        return "";
    }

    first_word(&s[first_word_len + 1..])
}
