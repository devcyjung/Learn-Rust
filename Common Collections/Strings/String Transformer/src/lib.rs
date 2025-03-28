pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

pub mod transformer {
    use super::Command;

    // TODO: Complete the function signature!
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        input
            .iter()
            .map(|x| match x.1 {
                Command::Append(times) => x.0.to_string() + &"bar".repeat(times),
                Command::Trim => x.0.trim().to_string(),
                Command::Uppercase => x.0.to_uppercase(),
            })
            .collect()
    }
}
