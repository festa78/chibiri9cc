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

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Reserved(ReservedKind), // symbol
    Num,                    // integer
    Eof,                    // end of token
}

#[derive(Debug, PartialEq, Clone)]
pub enum ReservedKind {
    Plus,
    Minus,
    Mul,
    Div,
    ParenLeft,
    ParenRight,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Larger,
    LargerEqual,
}

impl ReservedKind {
    pub fn len(&self) -> usize {
        match *self {
            ReservedKind::Plus => 1,
            ReservedKind::Minus => 1,
            ReservedKind::Mul => 1,
            ReservedKind::Div => 1,
            ReservedKind::ParenLeft => 1,
            ReservedKind::ParenRight => 1,
            ReservedKind::Equal => 2,
            ReservedKind::NotEqual => 2,
            ReservedKind::Less => 1,
            ReservedKind::LessEqual => 2,
            ReservedKind::Larger => 1,
            ReservedKind::LargerEqual => 2,
        }
    }

    pub fn str(&self) -> String {
        match *self {
            ReservedKind::Plus => '+'.to_string(),
            ReservedKind::Minus => '-'.to_string(),
            ReservedKind::Mul => '*'.to_string(),
            ReservedKind::Div => '/'.to_string(),
            ReservedKind::ParenLeft => '('.to_string(),
            ReservedKind::ParenRight => ')'.to_string(),
            ReservedKind::Equal => "==".to_string(),
            ReservedKind::NotEqual => "!=".to_string(),
            ReservedKind::Less => '<'.to_string(),
            ReservedKind::LessEqual => "<=".to_string(),
            ReservedKind::Larger => '>'.to_string(),
            ReservedKind::LargerEqual => ">=".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
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

pub fn consume_ops(token: &mut Token, reserved_kind: ReservedKind) -> bool {
    if token.kind == TokenKind::Reserved(reserved_kind) {
        let next_token = token.next.as_ref().unwrap().clone();
        token.kind = next_token.kind;
        token.next = next_token.next;
        token.str = next_token.str;
        token.location = next_token.location;
        return true;
    }
    false
}

pub fn expect_ops(token: &mut Token, reserved_kind: ReservedKind) -> Result<(), TokenizerError> {
    if token.kind != TokenKind::Reserved(reserved_kind) {
        return Err(TokenizerError::UnknownToken(token.location.clone()));
    }

    let next_token = token.next.as_ref().unwrap().clone();
    token.kind = next_token.kind;
    token.next = next_token.next;
    token.str = next_token.str;
    token.location = next_token.location;

    Ok(())
}

pub fn expect_number(token: &mut Token) -> Result<i32, TokenizerError> {
    if token.kind != TokenKind::Num {
        return Err(TokenizerError::UnknownToken(token.location.clone()));
    }

    let number = token.str.clone().unwrap().parse().unwrap();

    let next_token = token.next.as_ref().unwrap().clone();
    token.kind = next_token.kind;
    token.next = next_token.next;
    token.str = next_token.str;
    token.location = next_token.location;

    Ok(number)
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
        Some('*') => Some(ReservedKind::Mul),
        Some('/') => Some(ReservedKind::Div),
        Some('(') => Some(ReservedKind::ParenLeft),
        Some(')') => Some(ReservedKind::ParenRight),
        Some('=') => match chars.next() {
            Some('=') => Some(ReservedKind::Equal),
            _ => None,
        },
        Some('!') => match chars.next() {
            Some('=') => Some(ReservedKind::NotEqual),
            _ => None,
        },
        Some('<') => match chars.next() {
            Some('=') => Some(ReservedKind::LessEqual),
            _ => Some(ReservedKind::Less),
        },
        Some('>') => match chars.next() {
            Some('=') => Some(ReservedKind::LargerEqual),
            _ => Some(ReservedKind::Larger),
        },
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

pub fn tokenize(
    statement: std::rc::Rc<String>,
    start_index: usize,
) -> Result<Token, TokenizerError> {
    if statement.len() < start_index {
        return Err(TokenizerError::InvalidIndex(statement.len(), start_index));
    }

    if statement.len() == start_index {
        return Ok(Token {
            kind: TokenKind::Eof,
            next: None,
            str: None,
            location: statement::StatementWithLocation {
                statement: std::rc::Rc::clone(&statement),
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
        let next_token = tokenize(std::rc::Rc::clone(&statement), next_location)?;
        return Ok(Token {
            kind: TokenKind::Num,
            next: Some(Box::new(next_token)),
            str: Some(number),
            location: statement::StatementWithLocation {
                statement: std::rc::Rc::clone(&statement),
                index: start_index,
            },
        });
    }

    if let Some(ops) = pop_if_ops(&mut chars) {
        let next_location = start_index + ops.len();
        let next_token = tokenize(std::rc::Rc::clone(&statement), next_location)?;
        let ops_str = ops.str();
        return Ok(Token {
            kind: TokenKind::Reserved(ops),
            next: Some(Box::new(next_token)),
            str: Some(ops_str),
            location: statement::StatementWithLocation {
                statement: std::rc::Rc::clone(&statement),
                index: start_index,
            },
        });
    }

    Err(TokenizerError::UnknownToken(
        statement::StatementWithLocation {
            statement: std::rc::Rc::clone(&statement),
            index: start_index,
        },
    ))
}
