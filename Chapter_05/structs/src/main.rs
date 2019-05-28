struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = User {
        email : String::from("someone@example.com"),
        username: String::from("someusername123"),
        sign_in_count: 1,
        active: true,
    };

    println!("{}", user1.email);

    user1.username = String::from("someotheruser456");
    
    println!("{}", user1.username);

    let user2 = build_user(String::from("mailto@otherdomain.com"), String::from(("athirduser666")));

    println!("New user {} has the email {}", user2.username, user2.email);

    let user3 = User {
        email: String::from("noreply@google.com"),
        username: String::from("Bill Gates"),
        ..user1
    };

    println!("Third user {} has the email {}", user3.username, user3.email);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        sign_in_count: 0,
        active: false,
    }
}
