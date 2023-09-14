fn main() {
    
    let mut counter = 0;

    // loop is used to execute a block of code over and over again.
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
        
    };

    println!("The result is  -> {result}");


    // while loop
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("Outside the while loop!");


    // using while loop to iterate over Array
    let arr = [1, 2, 3, 4, 5];
    let mut index = 0;

    println!("Iterating over Array elements - ");
    while index < 5 {
        println!("The value is : {}", arr[index]);

        index += 1;
    }


    // for-loop to iterate over Array --> safe code over while-loop
    let a = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    for element in a {
        println!("The value is  -> {element}");
    }

    // another example of for-loop
    println!("Iterating over a range");
    for number in 1..4 {
        println!("{number}!");
    }

    println!("Iterating over a range in reverse");
    for number in (1..4).rev() {
        println!("{number}!");
    }
}
