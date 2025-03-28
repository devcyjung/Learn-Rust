// Lifetime of User = min(Lifetime of User.username, Lifetime of User.email)
#[derive(Debug)]
struct User<'a> {
    username: &'a str,
    email: &'a str,
    sign_in_count: u64,
    active: bool,
}

// Lifetime of return value = min(Lifetime of email, Lifetime of username)
fn build_user<'a>(email: &'a str, username: &'a str) -> User<'a> {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let mut user1 = User {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };

    user1.email = "anotheremail@example.com";
    let user2_email = "anyone@example.com";
    let user2_username = "anyone";
    let user2 = build_user(user2_email, user2_username);

    println!("User1:\n{:#?}\nUser2:\n{:#?}", user1, user2);
    println!("User2 Username: {} Email: {}", user2_username, user2_email);
    println!(
        "User1: {} {} {} {}",
        user1.username, user1.email, user1.sign_in_count, user1.active
    );
    println!(
        "User2: {} {} {} {}",
        user2.username, user2.email, user2.sign_in_count, user2.active
    );
}
