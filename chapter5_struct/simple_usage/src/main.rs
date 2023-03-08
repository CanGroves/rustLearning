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

    // using struct update syntax, we can achieve the same effect with less code.
    let user2 = User {
        email: String::from("email22@eg.com"),
        ..user1
    };
    // note: after using struct update syntax, fields which do not imp Copy trait (eg. username in this case) will be move to the new struct, thus the old struct can not be used after that
    
    
}


fn build_user(email: String, username: String) -> User {
    User { 
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}