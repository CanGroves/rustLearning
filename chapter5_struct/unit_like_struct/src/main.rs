struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual; // unit-like structs can be helpful when you need to implement a trait on some type but don't have any data that you want to store in the type itself.
}
