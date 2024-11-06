use std::io::Write;

fn main() {
    print!("Enter your name: ");
    std::io::stdout().flush().unwrap();

    let user = make_user();
    print_name(&user);
}

struct User {
    name: String,
}

fn print_name(user: &User) {
    println!("Name: {}", user.name);
}

fn make_user() -> User {
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).unwrap();

    User {
        name: name.trim().to_string(),
    }
}
