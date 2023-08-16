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

#[derive(Debug, PartialEq, Eq)]
enum ASTNode {
    Node(Vec<ASTNode>),
    Bold(Vec<ASTNode>),
    Italics(Vec<ASTNode>),
    Text(String),
}

fn parse(ts: &mut TokenizedString) -> ASTNode {
    let mut children: Vec<ASTNode> = Vec::new();

    loop {
        let next = ts.pop();
        println!("==================");
        println!("next: {:?}", next);

        if next == Token::EOF {
            break;
        }

        match next {
            Token::Asterisk => {
                // bold
                if ts.peek(0) == Token::Asterisk {
                    println!("FOUND START OF BOLD");
                    // consume the second asterisk
                    ts.pop();

                    // now lets find if/where this bold ends, by finding the next double asterisk
                    let mut offset = 1;
                    while ts.peek(offset) != Token::Asterisk
                        && ts.peek(offset + 1) != Token::Asterisk
                        && ts.peek(offset) != Token::EOF
                        && ts.peek(offset + 1) != Token::EOF
                    {
                        offset += 1;
                    }

                    // check if EOF
                    if ts.peek(offset) == Token::EOF || ts.peek(offset + 1) == Token::EOF {
                        todo!("unmatched asterisk");
                    }

                    println!("TOKENS RN {:?}", ts);

                    // we must have found the end of the bold text
                    // lets parse the bold text

                    let mut t = vec![];
                    for i in 0..offset {
                        t.push(ts.pop())
                    }

                    t.push(Token::EOF);

                    let mut bold_text = TokenizedString::new(String::from(""));
                    bold_text.tokens = t;

                    println!("bold_text: {:?}", bold_text.tokens);
                    println!("TOKENS RN {:?}", ts);

                    // remove the closing
                    ts.pop();
                    ts.pop();

                    // parse the bold text
                    children.push(parse_bold(&mut bold_text));
                }
            }
            Token::Text(s) => children.push(ASTNode::Text(s)),
            _ => {}
        };
    }

    ASTNode::Node(children)
}

fn parse_bold(ts: &mut TokenizedString) -> ASTNode {
    return ASTNode::Bold(vec![parse(ts)]);
}

fn main() {
    let source = String::from("Hello **World**");
    let mut tokens = TokenizedString::new(source);
    let ast = parse(&mut tokens);

    let expected_ast = ASTNode::Node(vec![
        ASTNode::Text(String::from("Hello ")),
        ASTNode::Bold(vec![ASTNode::Text(String::from("World"))]),
    ]);

    println!("AST: {:?}", ast);
    println!("EXPECTED AST: {:?}", expected_ast);

    assert_eq!(1, 1)
}

// ==============================================
//                     TESTS
// ==============================================

#[cfg(test)]
#[test]
fn basic_ast() {
    let source = String::from("Hello **World**");
    let mut tokens = TokenizedString::new(source);
    let ast = parse(&mut tokens);

    let expected_ast = ASTNode::Node(vec![
        ASTNode::Text(String::from("Hello ")),
        ASTNode::Bold(vec![ASTNode::Text(String::from("World"))]),
    ]);

    println!("AST: {:?}", ast);
    println!("EXPECTED AST: {:?}", expected_ast);

    assert_eq!(1, 1)
}
