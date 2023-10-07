fn main() {
    println!("Vectors in Rust!");

    /*
    Vectors allow you to store more than one value in a single data structure that puts 
    all the values next to each other in memory.
    */

    // 1st way of creating a new empty Vector
    let v: Vec<i32> = Vec::new();
    println!("Vector v is -> {:?}", v);

    // to add values to vector, it must be Mutable
    let mut v = Vec::new();
    v.push(5);
    v.push(10);
    v.push(2);
    v.push(41);

    // accessing elements of vector
    let third = &v[2];
    println!("Third element of vector is {}", third);

    // 2nd way of creating a new Vector with some initial values
    let v = vec![1, 2, 3];
    let third: Option<&i32> = v.get(2);
    match third {
        Some(val) => println!("The third value is {}", val),
        None => println!("There is no third element!"),
    }

    // iterate over vector using immutable reference
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![1, 2, 3];
    // iterate over vector using mutable reference
    // * -> dereference operator --> get value from reference
    for i in &mut v {
        *i += 50;
        println!("{i}");
    }

    // Vector with enum
    let row = vec![
        SpreadsheetCell::Int(5),
        SpreadsheetCell::Float(4.12),
        SpreadsheetCell::Text(String::from("Rust")),
    ];

    println!("Vector row is -> {:?}", row);

}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}