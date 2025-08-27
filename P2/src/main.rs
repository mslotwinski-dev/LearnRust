use std::io;

const OPERATORS: [char; 7] = ['+', '-', '*', '/', '(', ')', '='];

fn main() {
    println!("Witaj w kalkulatorze!");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Błąd odczytu danych");

    let mut tokens = format_tokens(input.clone());

    if !validate_tokens(&tokens) {
        println!("Błąd: nieprawidłowe tokeny");
        return;
    }

    tokens = parse_tokens(&mut tokens);

    println!("{}", tokens.join(" "));
}

fn format_tokens(input: String) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();

    let mut current_token = String::new();

    for c in input.trim().replace(" ", "").chars() {
        if OPERATORS.contains(&c) {
            tokens.push(current_token.clone());
            current_token.clear();
            tokens.push(c.to_string());
        } else {
            current_token.push(c);
        }
    }
    tokens.push(current_token.clone());
    return tokens;
}

fn parse_tokens(tokens: &mut Vec<String>) -> Vec<String> {
    let mut i = 0;

    while i < tokens.len() {
        if OPERATORS.contains(&tokens[i].as_str().chars().next().unwrap()) {
            let left = tokens[i - 1]
                .parse::<f64>()
                .expect("Błąd parsowania lewego argumentu");
            let right = tokens[i + 1]
                .parse::<f64>()
                .expect("Błąd parsowania prawego argumentu");

            match tokens[i].as_str() {
                "+" => tokens[i] = (left + right).to_string(),
                "-" => tokens[i] = (left - right).to_string(),
                "*" => tokens[i] = (left * right).to_string(),
                "/" => tokens[i] = (left / right).to_string(),
                _ => println!("Nieznany operator"),
            }

            tokens.remove(i - 1);
            tokens.remove(i);
            i = 0;
        }

        i += 1;

        if i >= tokens.len() {
            return tokens.to_vec();
        }
    }
    tokens.to_vec()
}

fn validate_tokens(tokens: &Vec<String>) -> bool {
    let mut paren_count = 0;
    let mut prev_token = "=";

    for token in tokens {
        if OPERATORS.contains(&prev_token.chars().next().unwrap())
            == OPERATORS.contains(&token.chars().next().unwrap())
        {
            return false;
        }
        if token == "(" {
            paren_count += 1;
        } else if token == ")" {
            paren_count -= 1;
        }
        prev_token = token;
    }
    return paren_count == 0;
}
