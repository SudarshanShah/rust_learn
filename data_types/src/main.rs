fn main() {
    // Scalar Data Types in Rust
    
    let x = 23; // by default Integer type is i32
    
    println!("Integer value is -> {x}");


    let x  = 2.0; // by default Floating type is f64

    println!("Floating value is -> {x}");



    let x = 20;
    let y = 10;

    let sum = x + y;
    println!("Addition -> {sum}");

    let difference = x - y;
    println!("Subtraction -> {difference}");

    let product = x * y;
    println!("Product -> {product}");

    let division = x / y;
    println!("Division -> {division}");

    let remainder = x % y;
    println!("Remainder -> {remainder}");

    let t = true;
    println!("True is -> {t}");

    let f = false;
    println!("False is -> {f}");

    let lower_char = 'a';
    println!("Lowercase character -> {lower_char}");

    let upper_char = 'A';
    println!("Uppercase character -> {upper_char}");



    // Compound Data Types in Rust

    // Tuples -> fixed size, can contain values of any data types
    let tup: (i32, f64, u8) = (11, 9.1, 10);

    // Tuple destructuring
    let (x, y, z) = tup;

    println!("The values in Tuple are  -> {x}, {y}, {z}");

    // accessing values using index
    let eleven = tup.0;
    let nine_point_one = tup.1;
    let ten = tup.2;

    println!("Tuple values using index -> {eleven}, {nine_point_one}, {ten}");

    // Note --> Empty tuples are also known as Unit.
    // Represented by () 
    // Expressions return empty tuple if they don't return any value.



    // Arrays -> fixed length, same data types
    let arr = [1, 2, 3, 4, 5];

    let first = arr[0];
    println!("First element of Array arr -> {first}");

    // it means array of length 5, all with value 3.
    let arr = [3; 5];

    let third = arr[2];
    println!("Third element of Array arr -> {third}");
}
