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
fn main() {
    let some_u8_value = Some(0u8);
    // match some_u8_value {
    //     Some(3) => println!("three"),
    //     // value => println!("{:?}", value), // value = 0
    //     _ => (),
    // }
    if let Some(3) = some_u8_value {
        println!("three");
    }

    let coin1 = Coin::Quarter(UsState::Alaska);
    let mut count = 0;
    match coin1 {
        Coin::Quarter(state) => println!("State quarter from {:?}", state),
        _ => count += 1,
    }

    let coin2 = Coin::Quarter(UsState::Alabama);

    if let Coin::Quarter(state) = coin2 {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
