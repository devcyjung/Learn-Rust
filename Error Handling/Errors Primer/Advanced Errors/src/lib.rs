use std::num::ParseIntError;
use std::str::FromStr;

// This is a custom error type that we will be using in the `FromStr`
// implementation.
#[derive(PartialEq, Debug)]
pub enum ParsePosNonzeroError {
    Creation(CreationError),
    ParseInt(ParseIntError),
}

impl From<CreationError> for ParsePosNonzeroError {
    fn from(e: CreationError) -> Self {
        Self::Creation(e)
    }
}

// TODO: implement another instance of the `From` trait here so that the
// `?` operator will work in the other place in the `FromStr`
// implementation below.
impl From<ParseIntError> for ParsePosNonzeroError {
    fn from(e: ParseIntError) -> Self {
        Self::ParseInt(e)
    }
}

// Don't change anything below this line.

impl FromStr for PositiveNonzeroInteger {
    type Err = ParsePosNonzeroError;
    fn from_str(s: &str) -> Result<PositiveNonzeroInteger, Self::Err> {
        let x: i64 = s.parse()?;
        Ok(PositiveNonzeroInteger::new(x)?)
    }
}

#[derive(PartialEq, Debug)]
pub struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
pub enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    pub fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            0 => Err(CreationError::Zero),
            x if x < 0 => Err(CreationError::Negative),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}
