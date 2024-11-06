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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_name() {
        let user = User {
            name: "Alice".to_string(),
        };

        print_name(&user);

        // 何を assert したらいい？
    }

    #[test]
    fn test_make_user() {
        let user = make_user();
        assert_eq!(user.name, "Alice");
    }
}
