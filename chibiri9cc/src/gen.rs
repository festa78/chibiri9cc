use thiserror::Error;

use super::parser;

#[derive(Error, Debug)]
pub enum GeneratorError {
    #[error("Unsupported node {:?}", .0)]
    UnsupportedNode(parser::NodeKind),
}

pub fn gen(node: parser::Node) -> Result<(), GeneratorError> {
    if node.kind == parser::NodeKind::Num {
        println!("  push {}", node.val.unwrap());
        return Ok(());
    }

    gen(*node.lhs.unwrap())?;
    gen(*node.rhs.unwrap())?;

    println!("  pop rdi");
    println!("  pop rax");

    match node.kind {
        parser::NodeKind::Add => println!("  add rax, rdi"),
        parser::NodeKind::Sub => println!("  sub rax, rdi"),
        parser::NodeKind::Mul => println!("  imul rax, rdi"),
        parser::NodeKind::Div => {
            println!("  cqo");
            println!("  idiv rdi");
        }
        _ => return Err(GeneratorError::UnsupportedNode(node.kind)),
    }

    println!("  push rax");

    Ok(())
}
