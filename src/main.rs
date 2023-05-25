mod locked;
mod user;

use locked::LockedType;
use user::User;

fn main() {
    let user = build_user(
        String::from("juliosperandio"),
        String::from("juliosperandio@hotmail.com"),
    );

    let mut var = String::from("");

    println!("Before: {}", var);

    execute(&mut var);

    println!("After: {}", var);

    write_user(user);
}

fn execute(variable: &mut String) {
    *variable = String::from("DragonicK");
}

fn build_user(username: String, email: String) -> User {
    return User {
        id: 1,
        username: username,
        email: email,
        sign_in_count: 1,
    };
}

fn write_user(user: User) {
    println!("Id: {}", user.id);
    println!("Username: {}", user.username);
    println!("Email: {}", user.email);
    println!("Sign In: {}", user.sign_in_count);
}
