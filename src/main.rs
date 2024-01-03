#[derive(Debug)]
enum Token {
    Identifier(String), // function name | variable name | let | const | etc...
    Number(String),     // 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | _
    Operator(String),   // = | + | - | * | / | etc...
    Delimiter(String),  // { | } | ( | ) | ; | etc...
    String(String),     // ' | "
    Template(String),   // `
}

#[derive(Debug)]
enum State {
    Initial,
    Identifier,
    Operator,
    Number,
    Delimiter,
    String(String),
}

fn lex(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut current = String::new();
    let mut chars = input.chars().peekable();
    let mut states: Vec<State> = vec![State::Initial];

    while let Some(&ch) = chars.peek() {
        match states.last() {
            Some(&State::Initial) => {
                match ch {
                    'a'..='z' | 'A'..='Z' | '_' => {
                        states.push(State::Identifier);
                    }
                    '=' => {
                        states.push(State::Operator);
                        tokens.push(Token::Operator(ch.to_string()));
                        chars.next();
                        states.push(State::Initial);
                    }
                    '0'..='9' => {
                        states.push(State::Number);
                    }
                    '.' | '(' | ')' | ';' | ',' | '{' | '}' | '[' | ']' | ':' => {
                        states.push(State::Delimiter);
                        tokens.push(Token::Delimiter(ch.to_string()));
                        chars.next();
                        states.push(State::Initial);
                    }
                    '\'' => {
                        states.push(State::String("single_quote".to_string()));
                        chars.next();
                    }
                    '"' => {
                        states.push(State::String("double_quote".to_string()));
                        chars.next();
                    }
                    '`' => {
                        states.push(State::String("back_quote".to_string()));
                        chars.next();
                    }
                    // ' ' | '\n' | '\t' => {
                    //     chars.next();
                    // },
                    _ => {
                        chars.next();
                    }
                }
            }
            Some(&State::Identifier) => match ch {
                'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => {
                    current.push(ch);
                    chars.next();
                }
                _ => {
                    tokens.push(Token::Identifier(current));
                    current = String::new();
                    states.push(State::Initial);
                }
            },
            Some(&State::Number) => match ch {
                '0'..='9' => {
                    current.push(ch);
                    chars.next();
                }
                '_' => {
                    chars.next();
                }
                _ => {
                    tokens.push(Token::Number(current));
                    current = String::new();
                    states.push(State::Initial);
                }
            },
            Some(&State::String(ref s)) => match ch {
                '\'' if s == "single_quote" => {
                    chars.next();
                    tokens.push(Token::String(current));
                    current = String::new();
                    states.push(State::Initial);
                }
                '"' if s == "double_quote" => {
                    chars.next();
                    tokens.push(Token::String(current));
                    current = String::new();
                    states.push(State::Initial);
                }
                '`' if s == "back_quote" => {
                    chars.next();
                    tokens.push(Token::Template(current));
                    current = String::new();
                    states.push(State::Initial);
                }
                _ => {
                    current.push(ch);
                    chars.next();
                }
            },
            _ => {
                chars.next();
            }
        }
    }
    tokens
}

// fn parse(tokens: Vec<Token>) -> AST {
//     // 将标记转换为 AST
// }

// fn execute(ast: AST, env: &mut Environment) {
//     // 执行 AST，更新环境
// }

fn main() {
    let file_path = "t.js";
    let input = std::fs::read_to_string(file_path);
    let input = match input {
        Ok(input) => input,
        Err(err) => {
            println!("Error: {}", err);
            return;
        }
    };
    let tokens = lex(input.as_str());
    // let ast = parse(tokens);
    // let mut env = Environment::new();
    // execute(ast, &mut env);
    print!("{:?}", tokens)
}
