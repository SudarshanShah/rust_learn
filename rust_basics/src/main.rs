fn main() {
    println!("Hello, world!");

    // immutable variable
    let apples = 5;

    println!("Apples count are -> {apples}");

    // mutable variable
    let mut mangoes = 2;

    println!("Mangoes count are -> {mangoes}");

    mangoes += 2;
    let total_fruits = apples + mangoes;

    println!("Total fruits are -> {total_fruits}");
}
