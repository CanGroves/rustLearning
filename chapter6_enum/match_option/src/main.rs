fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn print_str(x: Option<String>) {
    match x {
        None => println!("str is None"),
        Some(s) => println!("str is: {}", s),
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    println!("five: {:?}", five); // why was five not moved to function plus_one? because i32 is simple type, which implements Copy triat? yse! see test of String below:
    let str = Some(String::from("hello"));
    print_str(str);
    //println!("{:?}", str); // error! str was moved to function print_str, can not be used without clone() after that.

    let none = plus_one(None);
}
