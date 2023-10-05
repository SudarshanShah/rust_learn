struct User {
    _active: bool,
    username: String,
    email: String,
    _sign_in_count: u64
}

fn main() {
    println!("Hello, Structs!");

    // immutable instance of Struct
    let user1 = User {
        _active: true,
        username: String::from("sudarshan123"),
        email: String::from("sudarshan@gmail.com"),
        _sign_in_count: 1
    };

    println!("{}", user1.username);

    // mutable instance of Struct
    let mut user1 = User {
        _active: true,
        username: String::from("sudarshan123"),
        email: String::from("sudarshan@gmail.com"),
        _sign_in_count: 1
    };

    println!("{}", user1.email);

    user1.email = String::from("sudarshan123@gmail.com");
    println!("{}", user1.email);

    // function returning Struct
    let user2 = build_user(String::from("spiderman@avg.com"), String::from("peterparker"));
    println!("{}, {}", user2.email, user2.username);

    // verbose way to copy most of the details from other instance,
    // only few different values

    // let user2 = User {
    //     _active: user1._active,
    //     username: user1.username,
    //     email: String::from("stark@avg.com"),
    //     _sign_in_count: user1._sign_in_count
    // };

    // better way
    let user2 = User {
        email: String::from("stark@avg.com"),
        ..user1
    };

    println!("{}", user2.email);
}

fn build_user(email: String, username: String) -> User {
    User {
        _active: true,
        email,
        username,
        _sign_in_count: 2
    }
}