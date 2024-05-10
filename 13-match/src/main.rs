#[allow(dead_code)]

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(String),
}

fn value_of_coin(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(val) => {
            println!("Quarter of value is:- {}", val);
            25
        }
    }
}

fn main() {
    println!("Hello, world!");
    println!("{}", value_of_coin(Coin::Dime));
    println!("{}", value_of_coin(Coin::Quarter(String::from("fr"))));
}
