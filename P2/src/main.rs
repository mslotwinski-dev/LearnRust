use std::io;

#[derive(Debug, Clone)]
enum Token {
    Number(f64),
    Operator(char),
    LParen,
    RParen,
}

fn main() {
    println!("Witaj w kalkulatorze!");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Błąd odczytu danych");

    let tokens = tokenize(input);
    let rpn = to_rpn(tokens);
    let result = eval_rpn(rpn);

    println!("{:?}", result);
}

fn tokenize(input: String) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut number = String::new();

    for c in input.chars() {
        if c.is_ascii_digit() || c == '.' {
            number.push(c);
        } else {
            if !number.is_empty() {
                tokens.push(Token::Number(number.parse().unwrap()));
                number.clear();
            }
            match c {
                '+' | '-' | '*' | '/' => tokens.push(Token::Operator(c)),
                '(' => tokens.push(Token::LParen),
                ')' => tokens.push(Token::RParen),
                _ => {}
            }
        }
    }
    if !number.is_empty() {
        tokens.push(Token::Number(number.parse().unwrap()));
    }
    tokens
}

fn precedence(op: char) -> i32 {
    match op {
        '+' | '-' => 1,
        '*' | '/' => 2,
        _ => 0,
    }
}
fn to_rpn(tokens: Vec<Token>) -> Vec<Token> {
    let mut output = Vec::new();
    let mut ops = Vec::new();

    for token in tokens {
        match token {
            Token::Number(_) => output.push(token),
            Token::Operator(op) => {
                while let Some(Token::Operator(top)) = ops.last() {
                    if precedence(*top) >= precedence(op) {
                        output.push(ops.pop().unwrap());
                    } else {
                        break;
                    }
                }
                ops.push(Token::Operator(op));
            }
            Token::LParen => ops.push(Token::LParen),
            Token::RParen => {
                while let Some(top) = ops.pop() {
                    if let Token::LParen = top {
                        break;
                    }
                    output.push(top);
                }
            }
        }
    }
    while let Some(op) = ops.pop() {
        output.push(op);
    }
    output
}

fn eval_rpn(rpn: Vec<Token>) -> f64 {
    let mut stack = Vec::new();
    for token in rpn {
        match token {
            Token::Number(n) => stack.push(n),
            Token::Operator(op) => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                let result = match op {
                    '+' => a + b,
                    '-' => a - b,
                    '*' => a * b,
                    '/' => a / b,
                    _ => panic!("Nieznany operator"),
                };
                stack.push(result);
            }
            _ => {}
        }
    }
    stack[0]
}
