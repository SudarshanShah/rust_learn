fn main() {
    println!("References -> Slices");

    let s = String::from("hello world");

    let _hello = &s[0..5];
    let _world = &s[6..11];

    // if index starting from zero, we can write
    let hello = &s[..5];
    println!("{}", hello);
    // if index ending at last index, we can write
    let world = &s[6..];
    println!("{}", world);
    // if starting and ending index both included, write like this
    let str = &s[..];
    println!("{}", str);


    let first_word_in_string = first_word(&s);
    println!("{}", first_word_in_string);


    let using_string = first_word_better_version(&s);
    println!("{}", using_string);

    let str = "Hello Rust";
    let using_string_literals = first_word_better_version(str);
    println!("{}", using_string_literals);


    // Slices of array
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..3];
    assert_eq!(slice, &arr[1..3]);
}

// function to find first word in a string
fn first_word(s: &String) -> &str {
    // convert string to array of bytes
    let bytes = s.as_bytes();

    // i -> index, &item -> reference to each element
    // iter() -> returns each element in a collection
    // enumerate() -> wraps result of iter() to a tuple
    // first element of tuple is index, second is ref to element 
    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

// much better version would be to pass ref of string slice, not the ref of String
// so that we can take string literals also as parameters
fn first_word_better_version(s: &str) -> &str {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]
        }
    }

    &s[..]
}