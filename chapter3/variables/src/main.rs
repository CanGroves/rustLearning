fn main() {
    // let mut x = 5;
    // println!("x is {}", x);
    // x = 6;
    // println!("x is {}", x);
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("x in the inner scope is: {}", x);
    }

    println!("x in the outter scope is: {}", x);

    let spaces = "      ";
    let spaces = spaces.len();

    println!("len of spaces is: {}", spaces);

}
