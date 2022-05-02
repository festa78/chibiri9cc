use std::fmt;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum TokenError {
    #[error("Unknown token")]
    Unknown,
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Reserved, // symbol
    Num,      // integer
    Eof,      // end of token
}

pub struct Token {
    pub kind: TokenKind,
    pub next: Option<Box<Token>>,
    pub str: Option<String>,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let result = writeln!(
            f,
            "{:?}: {:?}",
            &self.kind,
            self.str.as_ref().unwrap_or(&"None".to_string())
        );
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
                } else if !number.is_empty() {
                    return Some(number);
                } else {
                    return None;
                }
            }
            _ => {
                if !number.is_empty() {
                    return Some(number);
                } else {
                    optional = None;
                }
            }
        };
    }
    None
}

pub fn tokenize(
    chars: &mut std::iter::Peekable<std::str::Chars>,
) -> Result<Token, impl std::error::Error> {
    if chars.clone().count() == 0 {
        return Ok(Token {
            kind: TokenKind::Eof,
            next: None,
            str: None,
        });
    }

    pop_if_space(chars);

    if let Some(number) = pop_if_number(chars) {
        return Ok(Token {
            kind: TokenKind::Num,
            next: Some(Box::new(tokenize(chars).unwrap())),
            str: Some(number),
        });
    }

    if let Some(ops) = pop_if_ops(chars) {
        return Ok(Token {
            kind: TokenKind::Reserved,
            next: Some(Box::new(tokenize(chars).unwrap())),
            str: Some(ops),
        });
    }

    Err(TokenError::Unknown)
}
