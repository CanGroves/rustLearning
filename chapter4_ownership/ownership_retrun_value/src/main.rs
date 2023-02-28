fn main() {
    let s1 = gives_ownership();

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);

} // s3 and s1 will be droped, but the ownership of s2 is moved into function takes_and_gives_back() and then is moved to s3 by returning value, so s2 will be invalid, and will not be droped.

fn gives_ownership() -> String {
    let some_string = String::from("yours");

    some_string
} // move the ownership to the caller by returning value, thus do not call drop() to the value some_string.

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}