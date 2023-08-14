use std::ops::Add;

#[derive(Debug)]
enum Token {
    Text(String),
    Asterisk,
    Underscore,
}

#[derive(Debug)]
struct TokenizedString {
    source: String,
    pub tokens: Vec<Token>,
}

impl TokenizedString {
    fn new(source: String) -> TokenizedString {
        TokenizedString {
            source: source.clone(),
            tokens: tokenize(source),
        }
    }
}

fn tokenize(source: String) -> Vec<Token> {
    // print the source
    println!("source: {:?}", source);

    // create a vector to hold the tokens

    let mut tokens = Vec::new();

    let mut window = String::new();

    let mut escape_next = false;
    // iterate over the characters in the source

    for char in source.chars() {
        if escape_next {
            window.push(char);
            escape_next = false;
            continue;
        }

        match char {
            '\\' => {
                escape_next = true;
            }
            '*' => {
                if window.len() > 0 {
                    tokens.push(Token::Text(window.clone()));
                    window.clear();
                }
                tokens.push(Token::Asterisk);
            }
            '_' => {
                if window.len() > 0 {
                    tokens.push(Token::Text(window.clone()));
                    window.clear();
                }
                tokens.push(Token::Underscore);
            }
            _ => {
                window.push(char);
            }
        }
    }

    if window.len() > 0 {
        tokens.push(Token::Text(window.clone()));
    }

    tokens
}

fn main() {
    let source = String::from("This is some **bold** and _italics_ and ***bold italics*** text.");

    let tokenized_struct = TokenizedString::new(source);
    println!("{:?}", tokenized_struct.tokens);
}
