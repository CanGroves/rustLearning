use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    // println!("The secret number is {}", secret_number);

    loop {
        println!("Please input ur guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        // let guess: u32 = guess.trim() // delete the spare empty char at the beginning and end of String. eg. \r\n in Windows, \n in Linux.
        //     .parse() // parse String to some type, specified by ': u32'. This method can get wrong by illegal string input, thus it will return 'Result' type, and we use expect() to handle it.
        //     .expect("Please type a number!"); // this var 'guess' shadow the var 'guess' defined in line 14, to allow programmers not to use too many vars to express the same meaning with different types, such as 'guess_str' and 'guess_u32'.
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed : {}", guess);

        match guess.cmp(&secret_number) { // rust can infer that secret_number should be a u32 by 'guess(u32).cmp()'.
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
    

}
