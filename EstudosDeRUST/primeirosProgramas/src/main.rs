struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: String::from("carlos@carlos.com"),
        username: String::from("Carlos"),
        active: true,
        sign_in_count: 1,
    };

    println!("{}", user1.email);
    println!("{}", user1.username);
    println!("{}", user1.active);
}
