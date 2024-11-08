use std::io::{BufRead, BufReader, Read, Write};

fn main() {
    // ここは一旦気にしなくて良い。
    print!("Enter your name: ");
    std::io::stdout().flush().unwrap();

    let user = make_user(&mut std::io::stdin());
    print_name(&mut std::io::stdout(), &user);
}

struct User {
    name: String,
}

/// ユーザーの名前を出力する。
fn print_name(w: &mut impl Write, user: &User) {
    // 標準出力してしまってる。
    writeln!(w, "Name: {}", user.name).unwrap();
}

/// 入力された名前を持つユーザーを作成する。
fn make_user(r: &mut impl Read) -> User {
    // 標準入力から名前を取得してしまってる。
    let mut name = String::new();
    let mut reader = BufReader::new(r);
    reader.read_line(&mut name).unwrap();

    User {
        name: name.trim().to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_name() {
        let mut writer = std::io::Cursor::new(Vec::new());
        let user = User {
            name: "Alice".to_string(),
        };

        print_name(&mut writer, &user);

        assert_eq!(
            String::from_utf8(writer.into_inner()).unwrap(),
            "Name: Alice\n"
        );
    }

    #[test]
    fn test_make_user() {
        let mut reader = std::io::Cursor::new("Alice".to_string().into_bytes());

        let user = make_user(&mut reader);
        assert_eq!(user.name, "Alice");
    }
}
