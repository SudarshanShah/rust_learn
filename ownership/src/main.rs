fn main() {
    println!("The Concept of Ownership!");

    // Rules of Ownership
    // 1. Each value in Rust has an owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    // ---- How Rust reclaim the memory------
    // When a variable goes out of scope,
    // Rust automatically calls the 'drop()' function
    // and cleans up the heap memory

    // String literals are immutable
    let _s = "hello";

    // we have String type for making mutable strings
    let mut s = String::from("hello");
    s.push_str(", world");  // method to append string
    println!("{}", s);


    // Here, value 5 is binded to variable x
    // And then, copy of value of x is binded to y
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    // here in case of String, 
    // on Stack -> we have pointer to memory that holds contents of String, 
    // a length, and a capacity ----- this all group of data is stored on stack
    // on Heap -> actual content is stored
    let s1 = String::from("Hello");
    // Here, String data on Stack is copied and allocated to s2
    // Heap content is not copied
    // s1 goes out of scope once s2 points to memory on Heap!!!!
    let s2 = s1;  // basically s1 was moved into s2

    // can't access s1 again -- compile-time error
    // println!("{}", s1);

    // to copy stack and heap data both, we can use clone() method
    let s1 = String::from("hello");
    let s2 = s1.clone();
    // here s1 and s2 points to different memory location with same value
    // clone() an expensive operation
    println!("s1 = {}, s2 = {}", s1, s2);


    /* 
     Any data type that implements 'Copy' trait,
     then, the varaible of that type do not move to another variable
     rather, it's value is copied to another variable

     Also, any data type that implements 'Drop' trait,
     is not allowed to implement 'Copy' trait

     'Copy' trat is implemented by all Scalar data types
     as well as Tuples if they contain types which implements 'Copy' trait
    */

    // Ownership and Functions
    // Passing a variable to a function will move or copy, just as assignment does.

    let s = String::from("hello");
    // s's value moves into the function
    // and s goes out of scope after that
    takes_ownership(s); 

    let x = 5;
    // x's value will move into the function,
    // but since i32 implements 'Copy' trait, we can use 'x' afterwards too.
    makes_copy(x);
     

    // Return values also transfer ownership
    let s1 = gives_ownership(); // return value moves into s1
    println!("s1 = {}", s1);

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);  // s2 moves into function & return value moves to s3
    println!("s3 = {}", s3);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string  = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}