fn main() {
    println!("Match control flow in Rust");

    println!(
        "Coin {:#?} in cents is {} cent",
        Coin::Penny,
        value_in_cents(Coin::Penny)
    );

    println!(
        "Coin {:#?} in cents is {} cent",
        Coin::Quarter(UsState::California),
        value_in_cents(Coin::Quarter(UsState::California))
    );

    println!("With value 5 -> {:?}", plus_one(Some(5)));
    println!("With value None -> {:?}", plus_one(None));


    // match for 'other' values
    fn a() -> u8 { 3 }
    fn b() -> u8 { 7 }
    fn c(x: u8) -> u8 { x }

    let dice_roll = 7;
    let res = match dice_roll {
        3 => a(),
        7 => b(),
        other => c(other),
    };
    println!("Result of dice roll is {}", res);


    // 1st way using match
    let value: Option<u8> = Some(3u8);
    match value {
        Some(v) => println!("Value is {}", v),
        _ => (),
    }

    // 2nd way using if-let
    if let Some(v) = value {
        println!("Value is {}", v);
    }

}

#[derive(Debug)]
enum Coin {
    Penny,
    _Nickel,
    _Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    California,
    _Alaska,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        }
        Coin::_Nickel => 5,
        Coin::_Dime => 10,
        Coin::Quarter(state) => {
            println!("State Quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}