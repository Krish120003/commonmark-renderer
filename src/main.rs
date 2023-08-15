#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    Text(String),
    Asterisk,
    Underscore,
    EOF,
}

#[derive(Debug)]
struct TokenizedString {
    pub tokens: Vec<Token>,
}

impl TokenizedString {
    fn new(source: String) -> TokenizedString {
        TokenizedString {
            tokens: tokenize(source),
        }
    }

    fn peek(&self, offset: usize) -> Token {
        self.tokens[offset].clone()
    }

    fn pop(&mut self) -> Token {
        self.tokens.remove(0)
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

    tokens.push(Token::EOF);

    tokens
}

enum AST {
    Body(Vec<AST>),
    Bold(Vec<AST>),
    Italics(Vec<AST>),
    Text(String),
}

fn parse(ts: &mut TokenizedString) {
    let children = Vec::new();

    let next = ts.pop();
    print!("next: {:?}", next);

    return match next {
        Token::EOF => {
            AST::Body(children);
        }
        Token::Asterisk => {
            // bold
            if ts.peek(0) == Token::Asterisk {
                // recursively parse bold text
            }
        }
        _ => {}
    };
}

fn parseBold(ts: &mut TokenizedString) {
    let children = Vec::new();

    let next = ts.pop();
    print!("next: {:?}", next);

    return match next {
        Token::EOF => {
            AST::Body(children);
        }
        Token::Asterisk => {
            // bold
            if ts.peek(0) == Token::Asterisk {
                // recursively parse bold text
            }
        }
        _ => {}
    };
}

fn main() {
    // let source = String::from("This is some **bold** and _italics_ and ***bold italics*** text.");

    // let tokenized_struct = TokenizedString::new(source);
    // println!("{:?}", tokenized_struct.tokens);

    let src = String::from("a **b**");
    let mut ts = TokenizedString::new(src);
    let res = parse(&mut ts);
    println!("{:?}", res);
}

#[cfg(test)]
#[test]
fn basic_ast() {
    let source = String::from("Hello **World**");
    let mut tokens = TokenizedString::new(source);
    let ast = parse(&mut tokens);

    let expected_ast = AST::Body(vec![
        AST::Text(String::from("Hello ")),
        AST::Bold(vec![AST::Text(String::from("World"))]),
    ]);

    assert_eq!(1, 1)
}
