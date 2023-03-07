fn main() {
    let mut s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    let word = first_world(&s);
    println!("the first word's index is: {}", word);

    //s.clear();
    // here 'word' is still 5

    let word = first_world_by_slice(&s);
    //s.clear(); // will cause an error when compling, use immutalbe borrow in line 13 and 15, and use mutable borrow in this line, which is between line 13 and 15, leading to exist immutable and mutable borrow at the same time!
    println!("the first word is: {}", word);

    // &str argument accepts slice of 'String', whether partial or whole
    let word = first_world_by_slice(&s[..6]);
    let word = first_world_by_slice(&s[..]);
    // &str argument also accepts ref of 'String', which equivalent to line 19
    let word = first_world_by_slice(&s);

    let my_string_literal = "hello world";

    // &str argument accepts slice of string literals, whether partial or whole
    let word = first_world_by_slice(&my_string_literal[0..6]);
    let word = first_world_by_slice(&my_string_literal[..]);

    // because string literals are string slices already, &str argument accepts it too, without slice syntax!
    let word = first_world_by_slice(my_string_literal);

}

fn first_world(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// old signatrue: fn first_world_by_slice(s: &String) -> &str {
// better signatrue of this kind of argument:
 fn first_world_by_slice(s: &str) -> &str {
// because '&str' can accept both kinds of String(by pass &String ot &xx[..]) and &str
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}