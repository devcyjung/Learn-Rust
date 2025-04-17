use std::iter::Filter;

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

fn shoe_in_my_size_v2<'a>(
    shoes: &'a [Shoe],
    shoe_size: &'a u32,
) -> Filter<impl Iterator<Item = &'a Shoe>, impl FnMut(&&'a Shoe) -> bool> {
    shoes.iter().filter(|s| s.size == *shoe_size)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_my_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );

        println!("First Assertion Passed!");

        let shoes_v2 = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size_v2 = shoe_in_my_size_v2(&shoes_v2, &10);
        assert_eq!(
            in_my_size_v2.collect::<Vec<_>>(),
            vec![
                &Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                &Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );

        println!("Second Assertion Passed!");
    }
}
