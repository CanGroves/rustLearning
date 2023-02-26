fn main() {
    let mut count = 0;

    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);

    return_from_loop();
    use_while();
    iterate_set_with_while(); // use while may cause index out of range.
    iterate_set_with_for(); // the better way to iterate set.
}

fn return_from_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result in return_from_loop is {}", result);
}

fn use_while() {
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!")
}

fn iterate_set_with_while() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value in iterate_set_with_while is: {}", a[index]);

        index += 1;
    }
}

fn iterate_set_with_for() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value in iterate_set_with_for is: {}", element);
    } 
}