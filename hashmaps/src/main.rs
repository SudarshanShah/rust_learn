use std::collections::HashMap;

fn main() {
    println!("HashMaps in Rust");

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 25);

    println!("The Hashmap scores is -> {:?}", scores);

    // accessing value using key
    let team_name = String::from("Blue");
    // get() --> takes ref of key, 
    // returns an Option containing an ref of value corresponding to Key
    // copied() -> converts Option<&T> to Option<T>
    // unwrap_or() -> gets the T from Option<T> or returns default if no value present
     let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("The score of Blue team is -> {score}");


    // Iterating HashMap
    for (key, value) in &scores {
        println!("{key} : {value}");
    }


    // Adding a Key and Value Only If a Key Isn’t Present
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Blue")).or_insert(20);
    scores.entry(String::from("Yellow")).or_insert(25);

    println!("{:?}", scores);

}
