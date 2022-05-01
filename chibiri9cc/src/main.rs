use std::fmt;

#[derive(Debug)]
enum TokenError {
    TKUnknown,
}

impl std::error::Error for TokenError {}

impl fmt::Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tokenize error: {:?}", self)
    }
}

#[derive(Debug, PartialEq)]
enum TokenKind {
    TkReserved,   // symbol
    TkNum,       // integer
    TkEof,       // end of token
}

struct Token {
    kind: TokenKind,
    next: Option<Box<Token>>,
    val: Option<i32>,
    str: Option<String>,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let result = write!(f, "{:?}: {:?}\n", &self.kind, self.str.as_ref().unwrap_or(&"None".to_string()));
        if let Some(next_token) = &self.next {
            return next_token.fmt(f);
        }
        result
    }
}

fn pop_if_space(chars: &mut std::iter::Peekable<std::str::Chars>) {
    if let Some(ops) = chars.peek() {
        if ops == &' ' {
            chars.next();
            pop_if_space(chars);
        }
    }
}

fn pop_if_ops(chars: &mut std::iter::Peekable<std::str::Chars>) -> Option<String> {
    if let Some(ops) = chars.peek() {
        if ops == &'+' || ops == &'-' {
            return Some(chars.next().unwrap().to_string());
        }
    }
    None
}

fn pop_if_number(chars: &mut std::iter::Peekable<std::str::Chars>) -> Option<String> {
    let mut optional = Some("".to_string());
    while let Some(mut number) = optional {
        match chars.peek() {
            Some(c) => {
                if c.is_digit(10) {
                    number.push(chars.next().unwrap());
                    optional = Some(number);
                } else if number.len() > 0{
                    return Some(number);
                } else {
                    return None;
                }
            }
            _ => {
                if number.len() > 0 {
                    return Some(number);
                } else {
                    optional = None;
                }
            }
        };
    }
    None
}


fn tokenize(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<Token, impl std::error::Error> {
    if chars.clone().count() == 0 {
        return Ok(Token{
            kind: TokenKind::TkEof,
            next: None,
            val: None,
            str: None,
        });
    }

    pop_if_space(chars);

    if let Some(number) = pop_if_number(chars) {
        return Ok(Token{
            kind: TokenKind::TkNum,
            next: Some(Box::new(tokenize(chars).unwrap())),
            val: Some(number.parse::<i32>().unwrap()),
            str: Some(number),
        });
    }

    if let Some(ops) = pop_if_ops(chars) {
        return Ok(Token{
            kind: TokenKind::TkReserved,
            next: Some(Box::new(tokenize(chars).unwrap())),
            val: None,
            str: Some(ops),
        });
    }

    Err(TokenError::TKUnknown)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Invalid number fo arguments");
        std::process::exit(1);
    }

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    let statement = args[1].to_string();
    let mut chars = statement.chars().peekable();
    let token = tokenize(&mut chars).unwrap();
    if &token.kind == &TokenKind::TkEof {
        panic!("no token found");
    }

    println!("  mov rax, {}", &token.str.expect("token should have str"));

    let mut next_ops_token = *token.next.expect("Invalide statement");

    while &next_ops_token.kind != &TokenKind::TkEof {
        if next_ops_token.kind != TokenKind::TkReserved {
            panic!("unknown token {}, expect ops", next_ops_token.str.expect("token should have str"));
        }

        let next_num_token = *next_ops_token.next.expect("Invalid statement");
        if next_num_token.kind != TokenKind::TkNum {
            panic!("unknown token {}, expect number", next_num_token.str.expect("token should have str"));
        }

        match next_ops_token.str.expect("token should have str").chars().next().unwrap() {
            '+' => println!("  add rax, {}", next_num_token.str.expect("token should have str")),
            '-' => println!("  sub rax, {}", next_num_token.str.expect("token should have str")),
            _ => panic!("unsupported ops {}", next_num_token.str.expect("token should have str")),
        }

        next_ops_token = *next_num_token.next.expect("Invalid statement");
    }

    println!("  ret");
}
