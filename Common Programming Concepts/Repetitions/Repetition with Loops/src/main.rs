fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2
        }
        
        if counter > 100 {
            // Every break statements should return a same type of value
            // break "42"; // wrong
            break 42
        }
    };

    println!("The result is {}", result);
    
    'label: for i in 1..10 {
        if i > 5 {
            // break with value is not allowed in for or while loops
            // break 42;
            break;
        } else if i == 3 {
            // break + label is ok
            break 'label
        } else {
            println!("i is {}", i);
        }
    };
}