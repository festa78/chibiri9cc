use thiserror::Error;

use super::tokenize;

#[derive(Error, Debug)]
pub enum CompileError {
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
}

pub fn compile(token: tokenize::Token) -> Result<(), CompileError> {
    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    if token.kind == tokenize::TokenKind::Eof {
        return Err(CompileError::NoTokenFound);
    }

    match &token.str {
        Some(str) => println!("  mov rax, {}", str),
        None => return Err(CompileError::StrAttrError(token.kind)),
    }

    if token.next.is_none() {
        return Err(CompileError::MissingEoF(token.kind));
    }
    let mut next_ops_token = *token.next.unwrap();

    while next_ops_token.kind != tokenize::TokenKind::Eof {
        if next_ops_token.next.is_none() {
            return Err(CompileError::MissingEoF(next_ops_token.kind));
        }
        let next_num_token = *next_ops_token.next.unwrap();

        if next_num_token.kind != tokenize::TokenKind::Num {
            return Err(CompileError::ExpectNum(next_num_token.kind));
        }

        if next_num_token.str.is_none() {
            return Err(CompileError::StrAttrError(next_num_token.kind));
        }

        if let tokenize::TokenKind::Reserved(reserved_ops) = &next_ops_token.kind {
            match reserved_ops {
                tokenize::ReservedKind::Plus => {
                    println!("  add rax, {}", next_num_token.str.unwrap())
                }
                tokenize::ReservedKind::Minus => {
                    println!("  sub rax, {}", next_num_token.str.unwrap())
                }
            }
        } else {
            return Err(CompileError::ExpectOps(next_ops_token.kind));
        }

        if next_num_token.next.is_none() {
            return Err(CompileError::MissingEoF(next_num_token.kind));
        }
        next_ops_token = *next_num_token.next.unwrap();
    }

    println!("  ret");
    Ok(())
}
