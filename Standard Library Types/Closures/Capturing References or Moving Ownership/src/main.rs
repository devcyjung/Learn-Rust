fn main() {
    // Defining and calling a closure that captures an immutable borrow
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");

    // Defining and calling a closure that captures a mutable borrow
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || list.push(7);

    // Cannot immutably borrow here because it's mutably borrowed in borrows_mutably
    // println!("Before calling closure: {list:?}");
    borrows_mutably();
    println!("After calling closure: {list:?}");

    let move_into = move || {
        println!("Move: {list:?}");
        list
    };
    // Cannot borrow here because it's moved into closure
    // println!("{list:?}");
    let new_list = move_into();
    // Cannot borrow here because it's moved
    // println!("{list:?}");

    println!("After move: {new_list:?}");
}
