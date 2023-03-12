fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => println!("run add_fancy_hat()!"),
        7 => println!("run remove_fancy_hat()!"),
        _ => println!("run reroll()!"),
        // _ => (), // to do nothing for all other cases.
        other => println!("run move_player({})!", other),
    }
}
