fn main() {
    println!("Ownership and References");

    // What is Reference?
    /*
         A reference is like a pointer in a sense that it’s an address we can follow to access 
         the data stored at that address; that data is owned by some other variable. 
         Unlike a pointer, a reference is guaranteed to point to a valid value of a 
         particular type for the life of that reference.

         Borrowing
         --> We call the action of creating a reference borrowing. 
         e.g., As in real life, if a person owns something, you can borrow it from them. 
         When you’re done, you have to give it back. You don’t own it.

         Rules of references:
         1. At any given time, you can have either one mutable reference or 
            any number of immutable references.
         2. References must always be valid.
     */

    // Without reference
    let s1 = String::from("hello");
    // s1 moves into function, and get out of scope
    // to use s1 value, we returning the string along with it's length as Tuple
    let (s, len) = calculate_length(s1);
    println!("s = {}, len = {}", s, len);
    


    // With reference
    let s1 = String::from("hello");
    // by passing reference of s1 to the function, ownership of value 'hello' remains with s1
    // but function can use the value, so s1 do not goes out of scope
    let len = calculate_length_ref(&s1);
    println!("s1 = {}, len = {}", s1, len);


    // mutable reference
    let mut s = String::from("hello");
    change(&mut s);

    // Mutable references have one big restriction: 
    // if you have a mutable reference to a value, 
    // you can have no other references to that value. 
    let mut s = String::from("value");
    let _r1 = &mut s; // it is valid
    //let r2 = &mut s; // it is not allowed
    //println!("r1 = {}, r2 = {}", r1, r2);

    // Restrictions on multiple mutable refs of same value at same time,
    // because of data races.

    // Another restriction
    // We also cannot have a mutable reference while we have an immutable one to the same value.
    // Users of an immutable reference don’t expect the value to suddenly change out from under them!
    let mut s = String::from("Rust");
    let _r1 = &s; // no problem
    let _r2 = &s; // no problem
    let _r3 = &mut s; // BIG PROBLEM


    // This is valid
    // once the immutable refs are used, we can afterwards declare mutable ref
    // but we need to make sure that after declaring mutable ref
    // we should not use immutable ref again
    let mut s = String::from("Rust");
    let _r1 = &s; // no problem
    let _r2 = &s; // no problem
    println!("{} and {}", _r1, _r2);

    let _r3 = &mut s; // no problem
    println!("{}", _r3);

    //let ref_to_nothing = dangle();  // compile time error

}

// s is of type String
fn calculate_length(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}

// s is of type 'reference to String' -> immutable reference
fn calculate_length_ref(s: &String) -> usize {
    s.len()
}

// s is of type 'reference to String' -> mutable reference
fn change(s: &mut String) {
    s.push_str(", world");
}

// it throws compile time error because
// we are passing the ref of local variable s, whose scope gets dropped after 
// function body ends, but we are returning ref of it, which will point to 
// invalid memory location.
// These things are not permissible in Rust
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }