use std::{
    fmt,
    io::{self, Write},
};

enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

impl<T> fmt::Display for Option<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Option::Some(t) => writeln!(f, "Some {}", t),
            Option::None => writeln!(f, "None"),
        }
    }
}

impl<T, E> fmt::Display for Result<T, E>
where
    T: fmt::Display,
    E: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Result::Ok(t) => writeln!(f, "Ok: {}", t),
            Result::Err(t) => writeln!(f, "Err: {:?}", t),
        }
    }
}

fn main() {
    // put you code here to launch it
    let o1 = Option::Some(5);
    let o2: Option<char> = Option::None;
    let o3 = Option::<char>::None;

    let r1: Result<_, io::Error> = Result::Ok(4);
    let r2 = Result::<char, io::Error>::Ok('ķ');
    let r3 = Result::<char, char>::Err('ñ');

    let mut stdout = io::stdout().lock();
    if let Err(e) = write!(stdout, "{}{}{}{}{}{}", o1, o2, o3, r1, r2, r3) {
        println!("Couldn't write to stdout: {:?}", e);
    }
}
