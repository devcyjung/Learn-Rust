fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    v.push(6);

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let mut v = vec![1, 2, 3, 4, 5];
    v.push(6);

    //let does_not_exist1 = &v[100]; // PANIC!!!
    let does_not_exist2 = v.get(100); // No panic
    match does_not_exist2 {
        Some(existing) => println!("The index 100 element is {}", existing),
        None => println!("There is no index 100 element."),
    }

    let range = 0..10;
    for n in range.clone().skip(1).step_by(2) {
        println!("{}", n);
    }

    println!();

    for n in range.step_by(2).rev() {
        println!("{}", n);
    }
}
