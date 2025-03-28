#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let user_email = "anyone@example.com".to_string();
    let user_username = "anyone".to_string();
    let user = build_user(user_email, user_username);

    println!("{}", user.username);
    println!("{:?}", user);
    // println!("{} {}", user_email, user_username);
}
