fn main() {
    let s = String::from("hello");

    takes_ownership(s);

    // println!("{}", s); // error: borrow of moved value: `s` value borrowed here after move

    let x = 5;

    makes_copy(x);
}

fn takes_ownership(some_string: String) { // some_string is not reference type, so this function take the ownership of String, and drop() it at the end of function scope.
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) { // i32 has fixed length, and has Copy() triat, so the ownership of it is not taken by this function, only makes a copy of it.
    println!("{}", some_integer);
}