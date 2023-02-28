fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of {} is {}.", s2, len); // can not use s1, because its ownership is move into function calculate_length, then it became invalid. so we use turple to get both the ownership of String and the lenth calculate by function.
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
