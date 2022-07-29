enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --
}

fn main() {
    let some_number = Some(5); // Option<i32>
    let some_string = Some("a string"); // Option<&str>

    let absent_number: Option<i32> = None;

    value_in_cents(Coin::Quarter(UsState::Alaska));

    let six = plus_one(some_number);
    let none = plus_one(absent_number);

    if let Some(six) = six {
        println!("Value is {}", six);
    }
    if let None = none {
        println!("Value is does not exist");
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
