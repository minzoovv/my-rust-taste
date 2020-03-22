struct User {
    username: String,
    email: String,
    active: bool,
}

struct Color(i32, i32, i32);

fn change_user_email(email: String, mut user: User) -> User {
    user.email = email;
    user
}

fn create_user(username: String, email: String) -> User {
    User {
        username,
        email,
        active: true,
    }
}

fn recreate_user(user: User) -> User {
    let user2 = User {
        email: String::from("new_email"),
        username: String::from("new_username"),
        ..user
    };

    user2
}

fn main() {
    // structure

    let mut user = User {
        username: String::from("minzoo"),
        email: String::from("kmju1997@gmail.com"),
        active: true,
    };
    user.email = String::from("kmju1997@naver.com");
    user.active = false;
    let new_user = change_user_email(String::from("new_email"), user);
    println!("{}", new_user.email);

    let mut a = String::from("hi"); // immutable이라 할당이 불가능
    a = String::from("other string");

    let user3 = recreate_user(new_user);
    println!("{}", user3.email);
    // tuple structure

    let black = Color(0, 0, 0);
    // let r, g, b = black.0, black.1, black.2;
}
