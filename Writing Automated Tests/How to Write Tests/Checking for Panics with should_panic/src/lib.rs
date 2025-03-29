use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub struct Guess {
    value: i32,
}

impl PartialOrd for Guess {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if (101..).contains(&value) {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        if (..=0).contains(&value) {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(101);
    }

    #[test]
    #[should_panic(expected = "Guess value must be greater than or equal to 1")]
    fn less_than_1() {
        Guess::new(0);
    }

    #[test]
    fn ge_1_and_le_100() {
        for i in 1..100 {
            let res = std::panic::catch_unwind(|| Guess::new(i));
            assert!(res.is_ok());
            assert_eq!(i, res.unwrap().value);
        }
    }
}
