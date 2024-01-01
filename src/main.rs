#[derive(Debug)]
enum Token {
    Keyword(String), // const, let...
    Identifier(String), // function name, variable name...
    Literal(String), // string, number, null, undefine, true, false...
    Operator(String), // =, +, -, *, /...
    Delimiter(String), // {, }, (, ), ;...
    TemplateLiteral(String), // `...`
    Arrow(String), // =>
    Whitespace(String), // 空格, 换行符...
    Deconstructor(String), // ...
}

enum State {
    Initial,
    Identifier,
}

fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut state = State::Initial;
    let mut current = String::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match state {
            State::Initial => {
                
            },
            State::Identifier => {
                
            },
            _ => {},
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
