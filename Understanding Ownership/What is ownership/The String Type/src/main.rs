fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`
    drop(s); // Automatically called when s goes out of the scope. s is moved into the drop function
    let v = "hihi";
    drop(v);
    let v1 = 1;
    drop(v1);
    println!("{}", v);
    println!("{}", v1);
}
