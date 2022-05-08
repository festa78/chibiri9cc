use thiserror::Error;

use super::parser;

#[derive(Error, Debug)]
pub enum GeneratorError {
    #[error("Unsupported node {:?}", .0)]
    UnsupportedNode(parser::NodeKind),
}

fn push() {
    println!("  push %rax");
}

fn pop(arg: String) {
    println!("  pop {}", arg);
}

pub fn gen(node: parser::Node) -> Result<(), GeneratorError> {
    match node.kind {
        parser::NodeKind::Num => {
            println!("  mov ${}, %rax", node.val.unwrap());
            return Ok(());
        }
        parser::NodeKind::Neg => {
            gen(*node.lhs.unwrap())?;
            println!("  neg %rax");
            return Ok(());
        }
        _ => {}
    }
    if node.kind == parser::NodeKind::Num {}

    gen(*node.rhs.unwrap())?;
    push();
    gen(*node.lhs.unwrap())?;
    pop("%rdi".to_string());

    match node.kind {
        parser::NodeKind::Add => println!("  add %rdi, %rax"),
        parser::NodeKind::Sub => println!("  sub %rdi, %rax"),
        parser::NodeKind::Mul => println!("  imul %rdi, %rax"),
        parser::NodeKind::Div => {
            println!("  cqo");
            println!("  idiv %rdi");
        }
        parser::NodeKind::Eq => {
            println!("  cmp %rdi, %rax");
            println!("  sete %al");
            println!("  movzb %al, %rax");
        }
        parser::NodeKind::Ne => {
            println!("  cmp %rdi, %rax");
            println!("  setne %al");
            println!("  movzb %al, %rax");
        }
        parser::NodeKind::Lt => {
            println!("  cmp %rdi, %rax");
            println!("  setl %al");
            println!("  movzb %al, %rax");
        }
        parser::NodeKind::Le => {
            println!("  cmp %rdi, %rax");
            println!("  setle %al");
            println!("  movzb %al, %rax");
        }
        _ => return Err(GeneratorError::UnsupportedNode(node.kind)),
    }

    Ok(())
}
