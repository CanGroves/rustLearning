
fn main() {
    let some_number = Some(5);
    let some_string = Some("a string");

    let abscent_number: Option<i32> = None; // when we use None, we should annotate the overall 'Option' type.
}
