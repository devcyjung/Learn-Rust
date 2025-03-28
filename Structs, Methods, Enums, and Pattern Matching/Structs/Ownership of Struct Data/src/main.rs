#[derive(Debug)]
struct User<'a> {
    username: &'a str, // !!! ERROR
    email: &'a str,    // !!! ERROR
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let _user = User {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };
    println!("{:?}", _user);
}
