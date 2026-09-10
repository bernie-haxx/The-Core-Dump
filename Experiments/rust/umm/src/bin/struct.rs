struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);

struct Point(i32, i32, i32);

struct AlwaysEqual;

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("berniehaxx"),
        email: String::from("bernie@gmail.com"),
        sign_in_count: 1,
    };

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);
    let username = String::from("beny");
    let email = String::from("umpalumpa@email.com");
    let mut build_user = build_user(username, email);

    let _subject = AlwaysEqual;

    build_user.username = String::from("why");
    user1.email = String::from("anotheremail@example.com");

    let _user2 = User {
        email: String::from("umpa@example.com"),
        ..user1
    };

    println!("{},{},{}", _black.0, _black.1, _black.2);
    println!("{},{},{}", _origin.0, _origin.1, _origin.2);
    println!("{}, {}", user1.active, user1.sign_in_count);
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
