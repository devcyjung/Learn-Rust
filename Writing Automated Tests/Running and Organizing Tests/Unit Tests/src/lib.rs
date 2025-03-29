pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_test() {
        let add_twos = add_two_helper();
        for (i, o) in add_twos {
            assert_eq!(add_two(*i), *o, "i: {}, o: {}", i, o);
        }
    }

    fn add_two_helper<'a>() -> &'a [(i32, i32)] {
        &[
            (1, 3),
            (2, 4),
            (3, 5),
            (100, 102),
            (0, 2),
            (-1, 1),
            (-3, -1),
            (-9999, -9997),
        ]
    }

    #[test]
    fn internal() {
        let internals = internal_helper();
        for (a, b, c) in internals {
            assert_eq!(internal_adder(*a, *b), *c, "a: {}, b: {}, c: {}", a, b, c);
        }
    }

    fn internal_helper<'a>() -> &'a [(i32, i32, i32)] {
        &[
            (1, 2, 3),
            (2, 3, 5),
            (3, 4, 7),
            (4, 7, 11),
            (5, 10, 15),
            (-23, 4, -19),
            (-13, -1, -14),
            (0, 3, 3),
            (-7, 0, -7),
        ]
    }
}
