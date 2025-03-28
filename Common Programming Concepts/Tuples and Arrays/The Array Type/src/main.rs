fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
    let [ one, two, three, four, five ] = a; 

    println!("The values of the first and second are: {} {}", first, second);
    println!("The values of one, two, three, four, five are: {} {} {} {} {}", one, two, three, four, five);
    
    let copy_of_one = [one; 5];
    println!("The values of the first and last elements of copy_of_one: {} {}", copy_of_one[0], copy_of_one[copy_of_one.len() - 1]);
    
    let months = ["January", "February", "March",
        "April", "May", "June",
        "July", "August", "September",
        "October", "November", "December"];

    println!("The last month is {}", months[months.len() - 1]);
}
