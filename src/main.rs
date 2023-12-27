#[derive(Debug, PartialEq)]
enum TokenType {
    Keyword,
    Identifier,
    Operator,
    Number,
    Semicolon,
}

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    value: String,
}

fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        let token = if ch.is_alphabetic() {
            let mut value = String::new();
            while let Some(&ch) = chars.peek() {
                if ch.is_alphanumeric() {
                    value.push(ch);
                    chars.next();
                } else {
                    break;
                }
            }

            match value.as_str() {
                "let" => Token { token_type: TokenType::Keyword, value },
                _ => Token { token_type: TokenType::Identifier, value },
            }
        } else if ch.is_digit(10) {
            let mut value = String::new();
            while let Some(&ch) = chars.peek() {
                if ch.is_digit(10) {
                    value.push(ch);
                    chars.next();
                } else {
                    break;
                }
            }

            Token { token_type: TokenType::Number, value }
        } else if ch == '=' {
            chars.next();
            Token { token_type: TokenType::Operator, value: ch.to_string() }
        } else if ch == ';' {
            chars.next();
            Token { token_type: TokenType::Semicolon, value: ch.to_string() }
        } else {
            // 简化处理：忽略其他字符
            chars.next();
            continue;
        };

        tokens.push(token);
    }

    tokens
}

fn main() {
    let input = "let x = 5;";
    let tokens = lex(input);
    println!("{:?}", tokens);
}
