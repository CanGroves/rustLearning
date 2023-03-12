
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip --
}
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin:: Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) =>{
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    let coin1 = Coin::Quarter(UsState::Alaska);
    value_in_cents(coin1);
    // println!("coint1 is {:?}", coin1); // error! coint1 was moved to function value_in_cents in line 33, can not use it after that.
}
