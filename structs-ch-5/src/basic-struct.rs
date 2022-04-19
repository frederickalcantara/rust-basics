struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// When we create a struct, we need to specify the key name and the type for its value

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // We don’t have to specify the fields in the same order in which we declared them in the struct. 
    // In other words, the struct definition is like a general template for the type, 
    // and instances fill in that template with particular data to create values of the type.
}
