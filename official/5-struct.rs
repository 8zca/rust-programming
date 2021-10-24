struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// タプル構造体
struct Point(usize, usize);

fn main() {
    let user1 = build_user(String::from("foo@example.com"), String::from("taro"));
    println!("{}", user1.username);

    let user2 = User {
        email: String::from("bar@example.com"),
        username: String::from("jiro"),
        // スプレッド構文的に利用できる(ドット2つ)
        ..user1
    };

    let point = Point(0, 0);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
