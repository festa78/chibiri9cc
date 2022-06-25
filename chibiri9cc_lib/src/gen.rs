use thiserror::Error;

use super::parser;

#[derive(Error, Debug)]
pub enum GeneratorError {
    #[error("Unsupported node {:?}", .0)]
    UnsupportedNode(parser::NodeKind),
}

fn push() -> String {
    "  push %rax\n".to_string()
}

fn pop(arg: String) -> String {
    format!("  pop {}\n", arg)
}

pub fn gen(node: parser::Node) -> Result<String, GeneratorError> {
    match node.kind {
        parser::NodeKind::Num => {
            return Ok(format!("  movq ${}, %rax\n", node.val.unwrap()));
        }
        parser::NodeKind::Neg => {
            return Ok(format!("{}\n  neg %rax\n", gen(*node.lhs.unwrap())?));
        }
        _ => {}
    }

    let mut generated = gen(*node.rhs.unwrap())?;
    generated += &push();
    generated += &gen(*node.lhs.unwrap())?;
    generated += &pop("%rdi".to_string());

    match node.kind {
        parser::NodeKind::Add => {
            generated += "  add %rdi, %rax\n";
        }
        parser::NodeKind::Sub => {
            generated += "  sub %rdi, %rax\n";
        }
        parser::NodeKind::Mul => {
            generated += "  imul %rdi, %rax\n";
        }
        parser::NodeKind::Div => {
            generated += "  cqo\n";
            generated += "  idiv %rdi\n";
        }
        parser::NodeKind::Eq => {
            generated += "  cmp %rdi, %rax\n";
            generated += "  sete %al\n";
            generated += "  movzb %al, %rax\n";
        }
        parser::NodeKind::Ne => {
            generated += "  cmp %rdi, %rax\n";
            generated += "  setne %al\n";
            generated += "  movzb %al, %rax\n";
        }
        parser::NodeKind::Lt => {
            generated += "  cmp %rdi, %rax\n";
            generated += "  setl %al\n";
            generated += "  movzb %al, %rax\n";
        }
        parser::NodeKind::Le => {
            generated += "  cmp %rdi, %rax\n";
            generated += "  setle %al\n";
            generated += "  movzb %al, %rax\n";
        }
        _ => return Err(GeneratorError::UnsupportedNode(node.kind)),
    }

    Ok(generated)
}
