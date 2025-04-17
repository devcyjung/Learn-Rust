pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    c.next().map_or_else(String::new, |first| {
        first.to_uppercase().chain(c).collect::<String>()
    })
}

// Step 2.
#[must_use]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    words.iter().map(|word| capitalize_first(word)).collect()
}

// This function is only needed to test your code through main.rs
pub fn iterate_string_vec() {
    let words = ["hello", "world"];
    let capitalized_words: Vec<String> =
        vec![capitalize_first(words[0]), capitalize_first(words[1])];
    println!("{capitalized_words:?}");
}

// Step 3.
#[must_use]
pub fn capitalize_words_string(words: &[&str]) -> String {
    // collect::<String>()
    // equal to
    // collect::<Vec<String>>().join("")
    words.iter().map(|word| capitalize_first(word)).collect()
}

// This function is only needed to test your code through main.rs
pub fn iterate_into_string() {
    let words = vec!["hello", " ", "world"];
    let capitalized_words = words.into_iter().map(capitalize_first).collect::<String>();
    println!("{capitalized_words:?}");
}
