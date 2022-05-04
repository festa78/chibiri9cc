use std::fmt;

use thiserror::Error;

#[derive(Error, Debug)]
enum TokenError {
    #[error("Unknown token")]
    Unknown,
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Reserved(ReservedKind), // symbol
    Num,                    // integer
    Eof,                    // end of token
}

#[derive(Debug, PartialEq)]
pub enum ReservedKind {
    Plus,
    Minus,
    Unknown,
}

impl ReservedKind {
    pub fn len(&self) -> Result<usize, impl std::error::Error> {
        match *self {
            ReservedKind::Plus => Ok(1),
            ReservedKind::Minus => Ok(1),
            _ => Err(TokenError::Unknown),
        }
    }

    pub fn str(&self) -> Result<String, impl std::error::Error> {
        match *self {
            ReservedKind::Plus => Ok('+'.to_string()),
            ReservedKind::Minus => Ok('-'.to_string()),
            _ => Err(TokenError::Unknown),
        }
    }
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub next: Option<Box<Token>>,
    pub str: Option<String>,
    pub location: usize,
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

fn pop_if_space(chars: &mut std::iter::Peekable<std::str::Chars>) -> usize {
    let mut num_spaces: usize = 0;
    if let Some(ops) = chars.peek() {
        if ops == &' ' {
            chars.next();
            num_spaces += 1 + pop_if_space(chars);
        }
    }
    num_spaces
}

fn pop_if_ops(chars: &mut std::iter::Peekable<std::str::Chars>) -> Option<ReservedKind> {
    match chars.next() {
        Some('+') => Some(ReservedKind::Plus),
        Some('-') => Some(ReservedKind::Minus),
        Some(_) => Some(ReservedKind::Unknown),
        None => None,
    }
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
    let mut current_idx: usize = 0;
    let chars_count = chars.clone().count();

    while current_idx < chars_count {
        let num_spaces = pop_if_space(chars);
        if num_spaces > 0 {
            current_idx += num_spaces;
            continue;
        }

        if let Some(number) = pop_if_number(chars) {
            let next_token = tokenize(chars).unwrap();
            current_idx += number.len();
            return Ok(Token {
                kind: TokenKind::Num,
                next: Some(Box::new(Token {
                    location: current_idx + next_token.location,
                    ..next_token
                })),
                str: Some(number),
                location: current_idx,
            });
        }

        if let Some(ops) = pop_if_ops(chars) {
            let next_token = tokenize(chars).unwrap();
            current_idx += ops.len().unwrap();
            let ops_str = ops.str().unwrap();
            return Ok(Token {
                kind: TokenKind::Reserved(ops),
                next: Some(Box::new(Token {
                    location: current_idx + next_token.location,
                    ..next_token
                })),
                str: Some(ops_str),
                location: current_idx,
            });
        }

        return Err(TokenError::Unknown);
    }

    Ok(Token {
        kind: TokenKind::Eof,
        next: None,
        str: None,
        location: current_idx,
    })
}
