struct User<'a> {
    active: bool,
    username: &'a str, // error: expected named lifetime parameter
    email: &'a str,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        email: "xxx",
        username: "sss",
        active: true,
        sign_in_count: 1,
    };
}
