use std::io::Write;

fn main() {
    // ここは一旦気にしなくて良い。
    print!("Enter your name: ");
    std::io::stdout().flush().unwrap();

    let user = make_user();
    print_name(&user);
}

struct User {
    name: String,
}

/// ユーザーの名前を出力する。
fn print_name(user: &User) {
    // 標準出力してしまってる。
    println!("Name: {}", user.name);
}

/// 入力された名前を持つユーザーを作成する。
fn make_user() -> User {
    // 標準入力から名前を取得してしまってる。
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).unwrap();

    User {
        name: name.trim().to_string(),
    }
}

// テストが書けない！！！！
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
