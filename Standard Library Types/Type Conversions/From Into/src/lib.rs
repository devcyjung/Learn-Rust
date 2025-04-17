#[derive(Debug)]
pub struct Person {
    pub name: String,
    pub age: usize,
}

impl Default for Person {
    fn default() -> Self {
        Self {
            name: String::from("John"),
            age: 30,
        }
    }
}

impl From<&str> for Person {
    fn from(s: &str) -> Self {
        match s {
            "" => Self::default(),
            _ => s
                .split_once(',')
                .map_or_else(Self::default, |(first, second)| {
                    if first.is_empty() {
                        return Self::default();
                    }
                    second.parse::<usize>().map_or(Self::default(), |age| Self {
                        name: String::from(first),
                        age,
                    })
                }),
        }
    }
}
