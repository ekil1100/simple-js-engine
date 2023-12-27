#[derive(Debug)]
enum Token {
    Keyword(String),
    Identifier(String),
    Number(i32),
    Operator(String),
    Semicolon,
    Dot,
    Parenthesis(char),
    // 其他标记...
}

enum State {
    Initial,
    Identifier,
    Number,
    Operator,
    // 其他状态...
}

fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut current_state = State::Initial;
    let mut current_token = String::new();

    for ch in input.chars() {
        match current_state {
            State::Initial => {
                current_state = State::Identifier;
            },
            State::Identifier => {
                if ch.is_alphabetic() {
                    current_token.push(ch);
                } else {
                    tokens.push(Token::Identifier(current_token.clone()));
                    current_token.clear();
                    current_state = State::Operator;
                }
            },
            State::Number => {
                if ch.is_numeric() {
                    current_token.push(ch);
                } else {
                    tokens.push(Token::Number(current_token.parse().unwrap()));
                    current_token.clear();
                    current_state = State::Initial;
                }
            },
            // 其他状态的处理...
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
    let input = "let x = 5; console.log(x);";
    let tokens = lex(input);
    // let ast = parse(tokens);
    // let mut env = Environment::new();
    // execute(ast, &mut env);
    print!("{:?}", tokens)
}
