fn main() {
    println!("Strings in Rust");

    // Strings are implemented as collection of bytes.
    // Strings are actually implemented as wrapper around a vector of bytes.

    // new empty, mutable string
    let s = String::new();
    println!("Empty string s is {s}");

    let data = "initial contents";

    // to_string() --> converts to String
    // only to those which implements Display trait.
    let s = data.to_string();
    println!("String with content -> {s}");

    // to_string() directly can apply on string literal.
    let s = "hello Rust".to_string();
    println!("String content is -> {s}");

    // the abvove code can be written in following way
    let s = String::from("initial content");
    println!("String content using from() is -> {s}");

    // Greeting in Hindi!!!
    let hello = String::from("नमस्ते");
    println!("{hello} Rust!");


    // Appending String
    let mut s = String::from("foo");
    // the method takes a string slice, so not necessarily wants ownership of value
    s.push_str("bar");
    println!("The appended string using push_str() is -> {s}");

    let mut s = String::from("lo");
    // this method accepts a single character
    s.push('l');
    println!("The new string formed using push() is -> {s}");


    // String concatenation with '+' operator
    let s1 = String::from("Hello, ");
    let s2 = String::from("world");
    // the '+' operator calls add() method, whose signature is like
    // fn add(self, s: &str) -> String
    // Also, &s2 is of type &String, not &str, but
    // Rust compiler 'coerce' &String to &str using 'deref coercion'
    // it turns &s2 into &s2[..] (a string slice). 
    let s3 = s1 + &s2;
    println!("The string concatenation result is {s3}");


    // format! macro for String concatenation
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // format! macro works like println! macro, 
    // but it returns String.
    // the format! macro uses references, so strings s1, s2, s3
    // remains valid afterwards also.
    let s = format!("{s1}-{s2}-{s3}");
    println!("String concatenation result using format! macro is -> {s}");

}
