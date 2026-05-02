struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u32,
}

fn main() {
    let user1 = build_user("Amanat", "amanat@gmail.com");
    
    println!("The name of the user is {}", user1.username);
    println!("The email of the user is {}", user1.email);
    println!("The active'nes of the user is {}", user1.active);
    println!("The sign_in_count of the user is {}", user1.sign_in_count);

    let user2 = User {
        email: String::from("alph702@gmail.com"),
        ..user1
    };

}

fn build_user(username: &str, email: &str) -> User {
    User {
        username: String::from(username),
        email: String::from(email),
        active: true,
        sign_in_count: 0,
    }
}