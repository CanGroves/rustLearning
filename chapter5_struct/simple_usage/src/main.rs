struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        email: String::from("emali1@eg.com"),
        username: String::from("name1"),
        active: true,
        sign_in_count:1,
    };

    user1.email = String::from("email2@eg.com");
}


fn build_user(email: String, username: String) -> User {
    User { 
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}