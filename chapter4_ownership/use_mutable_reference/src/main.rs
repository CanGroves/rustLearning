fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    let r1 = &mut s;
    println!("r1: {}", r1);
    let r2 = &mut s;
    println!("r2: {}", r2);

    {
        let r3 = &mut s;
    }
    let r4 = &mut s;

    // immutable
    let r5 = &s;
    let r6 = &s;
    // let r7 = &mut s; // error: cannot borrow `s` as mutable because it is also borrowed as immutable

    // println!("{}, {}, and {}", r5, r6, r7); // error: immutable borrow later used at r5.
    println!("{}, {}", r5, r6);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world.");
}
