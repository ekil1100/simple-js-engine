#[derive(Debug)]
enum Token {
    Identifier(String, Span), // function name | variable name | let | const | etc...
    Number(String, Span),     // 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | _
    Operator(String, Span),   // = | + | - | * | / | etc...
    Delimiter(String, Span),  // { | } | ( | ) | ; | etc...
    String(String, Span),     // ' | "
    Template(String, Span),   // `
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

#[derive(Debug, Copy, Clone)]
struct Position {
    line: usize,
    column: usize,
}

impl Position {
    fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}

#[derive(Debug, Copy, Clone)]
struct Span {
    start: Position,
    end: Position,
}

impl Span {
    fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }
}

fn lex(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut current = String::new();
    let mut chars = input.chars().peekable();
    let mut states: Vec<State> = vec![State::Initial];
    let mut column: usize = 1;
    let mut line: usize = 1;
    let mut start = Position::new(line, column);

    fn next_line(line: &mut usize, column: &mut usize) {
        *line += 1;
        *column = 0;
    }

    fn next_char(chars: &mut std::iter::Peekable<std::str::Chars>, column: &mut usize) {
        chars.next();
        *column += 1;
    }

    while let Some(&ch) = chars.peek() {
        match states.last() {
            Some(&State::Initial) => match ch {
                'a'..='z' | 'A'..='Z' | '_' => {
                    start = Position::new(line, column);
                    states.push(State::Identifier);
                }
                '=' | '+' | '-' | '*' | '/' => {
                    start = Position::new(line, column);
                    states.push(State::Operator);
                    tokens.push(Token::Operator(ch.to_string(), Span::new(start, start)));
                    next_char(&mut chars, &mut column);
                    states.push(State::Initial);
                }
                '0'..='9' => {
                    start = Position::new(line, column);
                    states.push(State::Number);
                }
                '.' | '(' | ')' | ';' | ',' | '{' | '}' | '[' | ']' | ':' => {
                    start = Position::new(line, column);
                    states.push(State::Delimiter);
                    tokens.push(Token::Delimiter(ch.to_string(), Span::new(start, start)));
                    next_char(&mut chars, &mut column);
                    states.push(State::Initial);
                }
                '\'' => {
                    start = Position::new(line, column);
                    states.push(State::String("single_quote".to_string()));
                    next_char(&mut chars, &mut column);
                }
                '"' => {
                    start = Position::new(line, column);
                    states.push(State::String("double_quote".to_string()));
                    next_char(&mut chars, &mut column);
                }
                '`' => {
                    start = Position::new(line, column);
                    states.push(State::String("back_quote".to_string()));
                    next_char(&mut chars, &mut column);
                }
                '\n' => {
                    next_line(&mut line, &mut column);
                    next_char(&mut chars, &mut column);
                }
                _ => {
                    next_char(&mut chars, &mut column);
                }
            },
            Some(&State::Identifier) => match ch {
                'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => {
                    current.push(ch);
                    next_char(&mut chars, &mut column);
                }
                _ => {
                    let end = Position::new(line, column);
                    tokens.push(Token::Identifier(current, Span::new(start, end)));
                    current = String::new();
                    states.push(State::Initial);
                }
            },
            Some(&State::Number) => match ch {
                '0'..='9' => {
                    current.push(ch);
                    next_char(&mut chars, &mut column);
                }
                '_' => {
                    next_char(&mut chars, &mut column);
                }
                _ => {
                    let end = Position::new(line, column);
                    tokens.push(Token::Number(current, Span::new(start, end)));
                    current = String::new();
                    states.push(State::Initial);
                }
            },
            Some(&State::String(ref s)) => match ch {
                '\'' if s == "single_quote" => {
                    next_char(&mut chars, &mut column);
                    let end = Position::new(line, column);
                    tokens.push(Token::String(current, Span::new(start, end)));
                    current = String::new();
                    states.push(State::Initial);
                }
                '"' if s == "double_quote" => {
                    next_char(&mut chars, &mut column);
                    let end = Position::new(line, column);
                    tokens.push(Token::String(current, Span::new(start, end)));
                    current = String::new();
                    states.push(State::Initial);
                }
                '`' if s == "back_quote" => {
                    next_char(&mut chars, &mut column);
                    let end = Position::new(line, column);
                    tokens.push(Token::Template(current, Span::new(start, end)));
                    current = String::new();
                    states.push(State::Initial);
                }
                _ => {
                    current.push(ch);
                    next_char(&mut chars, &mut column);
                }
            },
            _ => {
                next_char(&mut chars, &mut column);
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
