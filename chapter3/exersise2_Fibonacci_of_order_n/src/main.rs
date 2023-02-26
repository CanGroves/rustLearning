use std::io;

fn main() {
    println!("Fibonacci sequence of order n!");

    loop {
        println!("Type n:");

        let mut n = String::new();

        match io::stdin().read_line(&mut n) {
            Ok(_) => (),
            Err(_) => {
                println!("read_line failed!");
                continue;
            },
        }

        let n: u32 = match n.trim().parse() {
            Ok(num) => {
                if num >= 92 {
                    println!("Typing n >= 92! which will cause the result to be out of range of 'u64'. Please try again and type an unsigned integer number < 92!");
                    continue;
                } else {
                    num
                }
            },
            Err(_) => {
                println!("Typing n error! Please try again and type an unsigned integer number!");
                continue;
            },
        };

        print_fibonacci(n);
        println!("One loop end!");
    }

}

fn print_fibonacci(n: u32) {
    let mut n1: u64 = 1;
    let mut n2: u64 = 1;
    print!("The fibonacci sequence of order {} is: ", n);
    if n == 0 {
        println!("order 0 has no sequence!");
    }
    for i in 0..n {
        let tmp = n2;
        print!("{}", n1);
        if i != n-1 {
            print!(", ");
        } else {
            println!("");
        }
        n2 += n1;
        n1 = tmp;
    }
}
