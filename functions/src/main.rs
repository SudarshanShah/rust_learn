fn main() {
    println!("Hello, world!");

    another_function();

    another_function_with_parameter(11);

    print_labeled_measurement(9, 'h');

    // Block of code as expression
    // the line that doesn't ends with semicolon is an return value, an expression.
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y  is {y}");

    let x = five();
    println!("The function five() returned the value -> {x}");

    let x = plus_one(6);
    println!("The function plus_one() return -> {x}");
}

fn another_function() {
    println!("Another function");
}

fn another_function_with_parameter(x: i32) {
    println!("The value of x is -> {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is : {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}