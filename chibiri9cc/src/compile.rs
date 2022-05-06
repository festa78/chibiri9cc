use thiserror::Error;

use super::tokenize;

#[derive(Error, Debug)]
enum ParseError {
    #[error("No token found")]
    NoTokenFound,
    #[error("str attribute required for `{:?}`", .0)]
    StrAttrError(tokenize::TokenKind),
    #[error("Token does not finish with EoF but `{:?}`", .0)]
    MissingEoF(tokenize::TokenKind),
    #[error("Expect ops token but get `{:?}`", .0)]
    ExpectOps(tokenize::TokenKind),
    #[error("Expect number token but get `{:?}`", .0)]
    ExpectNum(tokenize::TokenKind),
    #[error("Unsupported ops `{:?}`", .0)]
    UnsupportedOps(tokenize::TokenKind),
}

pub fn compile(token: tokenize::Token) -> Result<(), impl std::error::Error> {
    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    if token.kind == tokenize::TokenKind::Eof {
        return Err(ParseError::NoTokenFound);
    }

    match &token.str {
        Some(str) => println!("  mov rax, {}", str),
        None => return Err(ParseError::StrAttrError(token.kind)),
    }

    if token.next.is_none() {
        return Err(ParseError::MissingEoF(token.kind));
    }
    let mut next_ops_token = *token.next.unwrap();

    while next_ops_token.kind != tokenize::TokenKind::Eof {
        if next_ops_token.next.is_none() {
            return Err(ParseError::MissingEoF(next_ops_token.kind));
        }
        let next_num_token = *next_ops_token.next.unwrap();

        if next_num_token.kind != tokenize::TokenKind::Num {
            return Err(ParseError::ExpectNum(next_num_token.kind));
        }

        if next_num_token.str.is_none() {
            return Err(ParseError::StrAttrError(next_num_token.kind));
        }

        if let tokenize::TokenKind::Reserved(reserved_ops) = &next_ops_token.kind {
            match reserved_ops {
                tokenize::ReservedKind::Plus => {
                    println!("  add rax, {}", next_num_token.str.unwrap())
                }
                tokenize::ReservedKind::Minus => {
                    println!("  sub rax, {}", next_num_token.str.unwrap())
                }
                _ => return Err(ParseError::UnsupportedOps(next_ops_token.kind)),
            }
        } else {
            return Err(ParseError::ExpectOps(next_ops_token.kind));
        }

        if next_num_token.next.is_none() {
            return Err(ParseError::MissingEoF(next_num_token.kind));
        }
        next_ops_token = *next_num_token.next.unwrap();
    }

    println!("  ret");
    Ok(())
}
