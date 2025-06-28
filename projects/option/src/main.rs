#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from: {state:?}");
            25
        }
    }
}

fn main() {
    let value = value_in_cents(Coin::Quarter(UsState::Alaska));

    println!("The value of the coin is: {}", value);

    // let x: i8 = 5;
    // let y: Option<i8> = Some(10);

    // let sum = x + y;

    // println!("The sum is: {}", sum);
}
