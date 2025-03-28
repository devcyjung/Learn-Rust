fn main() {
    for number in 1..5 { // 5 is excluded
        println!("{}", number);
    }

    for number in 1..=5 { // 5 is included (note '=')
        println!("{}", number);
    }

    for number in (1..4).rev() {// 3, 2, 1
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
    
    for number in (1..=3).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}