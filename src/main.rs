struct User {
    email:String,
    username:String,
    active: bool,
    sign_in_count: u32,
}
fn main() {
    let email = String::from("email");
    let username = String::from("username");

    let mut user = build_user(email,username);
    println!("{}{}",user.username, user.email);

    user.username = String::from("username2");
    println!("{}",user.username)
    
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
