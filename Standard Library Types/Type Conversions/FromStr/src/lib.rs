use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct Person {
    pub name: String,
    pub age: usize,
}

// We will use this error type for the `FromStr` implementation.
#[derive(Debug, PartialEq)]
pub enum ParsePersonError {
    // Empty input string
    Empty,
    // Incorrect number of fields
    BadLen,
    // Empty name field
    NoName,
    // Wrapped error from parse::<usize>()
    ParseInt(ParseIntError),
}

impl From<ParseIntError> for ParsePersonError {
    fn from(err: ParseIntError) -> Self {
        Self::ParseInt(err)
    }
}

impl FromStr for Person {
    type Err = ParsePersonError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(Self::Err::Empty);
        }
        let parts = s.split(',').collect::<Vec<_>>();
        if parts.len() != 2 {
            return Err(Self::Err::BadLen);
        }
        let name = if parts[0].is_empty() {
            return Err(Self::Err::NoName);
        } else {
            String::from(parts[0])
        };
        let age = parts[1].parse::<usize>()?;
        Ok(Self { name, age })
    }
}
