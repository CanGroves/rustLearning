use std::io;

fn main() {
    println!("Degree translation!");

    loop {
        println!("Type your origin kind of degree!('F' for degree Fahrenheit, 'C' for degree Celsius):");

        let mut kind = String::new();

        match io::stdin().read_line(&mut kind) {
            Ok(_) => {
                match kind.trim() {
                    "F" => println!("The origin kind you type is 'F', which means that you mean to translate from degree Fahrenheit to degree Celsius!"),
                    "C" => println!("The origin kind you type is 'C', which means that you mean to translate from degree Celsius to degree Fahrenheit!"),
                    _ => {
                        println!("Typing kind error! Please try again and type one of them: 'F' or 'C'");
                        continue;
                    },
                }
            },
            Err(_) => println!("read_line failed!"),
        }

        println!("Type your origin degree number:");

        let mut degree = String::new();

        match io::stdin().read_line(&mut degree) {
            Ok(_) => println!("The origin degree you type is {}", degree),
            Err(_) => println!("read_line failed!"),
        }

        let degree: f64 = match degree.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Typing degree error! Please try again and type a float number!");
                continue;
            },
        };

        let result = translation(degree, &kind.trim());

        println!("The result is: {} degree.", result);
    }
}

fn translation(degree: f64, from_kind: &str) -> f64 {
    match from_kind {
        "F" => (degree - 32.0) / 1.8,
        "C" => degree * 1.8 + 32.0,
        _ => {
            println!("Typing kind error! Please try again and type one of them: 'F' or 'C'");
            0.0
        },
    }
}