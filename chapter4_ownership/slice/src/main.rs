fn main() {
    let mut s = String::from("hello world");

    let word = first_world(&s);

    s.clear();
    // here 'word' is still 5
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