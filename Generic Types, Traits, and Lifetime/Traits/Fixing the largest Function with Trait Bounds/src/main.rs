fn largest<T: PartialOrd + Copy>(list: &[T]) -> Option<T> {
    let mut largest = *list.first()?;

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    Some(largest)
}

fn largest_v2<T: PartialOrd + Clone>(list: &[T]) -> Option<T> {
    let mut largest = list.first()?.clone();
    for item in list {
        if item > &largest {
            largest = item.clone();
        }
    }
    Some(largest)
}

fn largest_v3<T: PartialOrd>(list: &[T]) -> Option<&T> {
    let mut largest = list.first()?;
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    Some(largest)
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {:?}", result);
    let result = largest_v2(&number_list);
    println!("The largest number is {:?}", result);
    let result = largest_v3(&number_list);
    println!("The largest number is {:?}", result);

    let char_list = vec!['y', 'm', 'a', '한', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {:?}", result);
    let result = largest_v2(&char_list);
    println!("The largest char is {:?}", result);
    let result = largest_v3(&char_list);
    println!("The largest char is {:?}", result);

    let str_list = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
    ];
    let result = largest(&str_list);
    println!("The largest string is {:?}", result);
    let result = largest_v2(&str_list);
    println!("The largest string is {:?}", result);
    let result = largest_v3(&str_list);
    println!("The largest string is {:?}", result);

    let empty: [u32; 0] = [];
    let result = largest(&empty);
    println!("The largest empty vector element is {:?}", result);
    let result = largest_v2(&empty);
    println!("The largest empty vector element is {:?}", result);
    let result = largest_v3(&empty);
    println!("The largest empty vector element is {:?}", result);
}
