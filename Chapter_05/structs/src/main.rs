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
}
