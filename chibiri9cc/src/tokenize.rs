use std::fmt;

use thiserror::Error;

use super::statement;

#[derive(Error, Debug)]
pub enum TokenizerError {
    #[error("Start index is invalid. The statement has length {:?} but got intex {:?}", .0, .1)]
    InvalidIndex(usize, usize),
    #[error("{}Unknown token found", .0.str())]
    UnknownToken(statement::StatementWithLocation),
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
}

impl ReservedKind {
    pub fn len(&self) -> usize {
        match *self {
            ReservedKind::Plus => 1,
            ReservedKind::Minus => 1,
        }
    }

    pub fn str(&self) -> String {
        match *self {
            ReservedKind::Plus => '+'.to_string(),
            ReservedKind::Minus => '-'.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub next: Option<Box<Token>>,
    pub str: Option<String>,
    pub location: statement::StatementWithLocation,
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
        _ => None,
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

pub fn tokenize(statement: &str, start_index: usize) -> Result<Token, TokenizerError> {
    if statement.len() < start_index {
        return Err(TokenizerError::InvalidIndex(statement.len(), start_index));
    }

    if statement.len() == start_index {
        return Ok(Token {
            kind: TokenKind::Eof,
            next: None,
            str: None,
            location: statement::StatementWithLocation {
                statement: statement.to_string(),
                index: start_index,
            },
        });
    }
    let mut chars = statement[start_index..].chars().peekable();
    let num_spaces = pop_if_space(&mut chars);
    if num_spaces > 0 {
        return tokenize(statement, start_index + num_spaces);
    }

    if let Some(number) = pop_if_number(&mut chars) {
        let next_location = start_index + number.len();
        let next_token = tokenize(statement, next_location)?;
        return Ok(Token {
            kind: TokenKind::Num,
            next: Some(Box::new(next_token)),
            str: Some(number),
            location: statement::StatementWithLocation {
                statement: statement.to_string(),
                index: start_index,
            },
        });
    }

    if let Some(ops) = pop_if_ops(&mut chars) {
        let next_location = start_index + ops.len();
        let next_token = tokenize(statement, next_location)?;
        let ops_str = ops.str();
        return Ok(Token {
            kind: TokenKind::Reserved(ops),
            next: Some(Box::new(next_token)),
            str: Some(ops_str),
            location: statement::StatementWithLocation {
                statement: statement.to_string(),
                index: start_index,
            },
        });
    }

    Err(TokenizerError::UnknownToken(
        statement::StatementWithLocation {
            statement: statement.to_string(),
            index: start_index,
        },
    ))
}
