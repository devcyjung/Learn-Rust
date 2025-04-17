use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq, Eq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

// We will use this error type for these `TryFrom` conversions.
#[derive(Debug, PartialEq, Eq)]
pub enum IntoColorError {
    // Incorrect length of slice
    BadLen,
    // Integer conversion error
    IntConversion,
}

// Tuple implementation
impl TryFrom<(i16, i16, i16)> for Color {
    type Error = IntoColorError;
    fn try_from(tuple: (i16, i16, i16)) -> Result<Self, Self::Error> {
        let r0: Result<u8, _> = tuple.0.try_into();
        let r1: Result<u8, _> = tuple.1.try_into();
        let r2: Result<u8, _> = tuple.2.try_into();
        if let (Ok(red), Ok(green), Ok(blue)) = (r0, r1, r2) {
            return Ok(Self { red, green, blue });
        }
        Err(Self::Error::IntConversion)
    }
}

// Array implementation
impl TryFrom<[i16; 3]> for Color {
    type Error = IntoColorError;
    fn try_from(arr: [i16; 3]) -> Result<Self, Self::Error> {
        let r0: Result<u8, _> = arr[0].try_into();
        let r1: Result<u8, _> = arr[1].try_into();
        let r2: Result<u8, _> = arr[2].try_into();
        if let (Ok(red), Ok(green), Ok(blue)) = (r0, r1, r2) {
            return Ok(Self { red, green, blue });
        }
        Err(Self::Error::IntConversion)
    }
}

// Slice implementation
impl TryFrom<&[i16]> for Color {
    type Error = IntoColorError;
    fn try_from(slice: &[i16]) -> Result<Self, Self::Error> {
        if slice.len() != 3 {
            return Err(Self::Error::BadLen);
        }
        let r0: Result<u8, _> = slice[0].try_into();
        let r1: Result<u8, _> = slice[1].try_into();
        let r2: Result<u8, _> = slice[2].try_into();
        if let (Ok(red), Ok(green), Ok(blue)) = (r0, r1, r2) {
            return Ok(Self { red, green, blue });
        }
        Err(Self::Error::IntConversion)
    }
}
