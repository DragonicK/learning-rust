mod locked;
mod user;

use locked::LockedType;
use user::User;

fn main() {
    let user = build_user(
        String::from("juliosperandio"),
        String::from("juliosperandio@hotmail.com"),
    );

    let locked = LockedType::Locked;

    let r = convert_from_u32_nightly(32);

    println!("Returned {}.", r);

    write_user(user);
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

fn convert_from_u32_nightly(input: u32) -> i32 {
    if let Ok(response) = i32::try_from(input) {
        return response;
    } else {
        return 0;
    };
}
