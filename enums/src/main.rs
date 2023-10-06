fn main() {
    println!("Enums in Rust");

    // instances of enum
    let four = IpAddrKind::V4;
    println!("{:#?}", four);
    let six = IpAddrKind::V6;
    println!("{:#?}", six);

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
    println!("V4 -> {:#?} and V6 -> {:#?}", home, loopback);


    // enum with variants having different data types
    #[derive(Debug)]
    enum Message{
        _Quit,
        _Move{x : i32, y : i32},
        Write(String),
        _ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            println!("Inside call() method");
        }
    }

    let m = Message::Write(String::from("Hello"));
    m.call();

    // Option enum  -> Special
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;
    println!("{:?}, {:?}, {:?}", some_number, some_char, absent_number);


}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

// enum variants and values associated with them
#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

