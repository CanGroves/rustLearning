/*
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
} // s leave its scope, whose memory will be released
// so out of the scope, &s will reference to an invalid String, which be a dangling pointer!
 */

fn main() {
    let borrowed = no_dangle();
}

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}