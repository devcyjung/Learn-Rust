use std::convert::{TryFrom, TryInto};
use try_from_into::Color;

fn main() {
    // Use the `from` function
    let c1 = Color::try_from((183, 65, 14));
    println!("{c1:?}");

    // Since TryFrom is implemented for Color, we should be able to use TryInto
    let c2: Result<Color, _> = [183, 65, 14].try_into();
    println!("{c2:?}");

    let mut v = vec![183, 65, 14];
    // With slice we should use `try_from` function
    let c3 = Color::try_from(&v[..]);
    println!("{c3:?}");
    // or take slice within round brackets and use TryInto
    let c4: Result<Color, _> = (&v[..]).try_into();
    println!("{c4:?}");

    v[2] = 0x7FFF;
    let c5: Result<Color, _> = (&v[..]).try_into();
    println!("{c5:?}");

    v.push(23);
    let c6: Result<Color, _> = (&v[..]).try_into();
    println!("{c6:?}");
}
