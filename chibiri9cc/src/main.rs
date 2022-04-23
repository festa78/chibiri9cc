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
    let first_number =
        pop_next_number(&mut chars).expect(format!("Invalid statement {}", &statement).as_str());
    println!("  mov rax, {}", first_number);

    while let Some(ops) = pop_next_ops(&mut chars) {
        match ops {
            Operation::Plus => println!(
                "  add rax, {}",
                pop_next_number(&mut chars)
                    .expect(format!("Invalid statement {}", &statement).as_str())
                    .as_str()
            ),
            Operation::Minus => println!(
                "  sub rax, {}",
                pop_next_number(&mut chars)
                    .expect(format!("Invalid statement {}", &statement).as_str())
                    .as_str()
            ),
        };
    }
    println!("  ret");
}

#[derive(PartialEq, Eq)]
enum Operation {
    Plus,
    Minus,
}

fn pop_next_ops(chars: &mut std::iter::Peekable<std::str::Chars>) -> Option<Operation> {
    if let Some(ops) = chars.next() {
        if ops == '+' {
            return Some(Operation::Plus);
        } else if ops == '-' {
            return Some(Operation::Minus);
        }
    }
    None
}

fn pop_next_number(chars: &mut std::iter::Peekable<std::str::Chars>) -> Option<String> {
    let mut optional = Some("".to_string());
    while let Some(mut number) = optional {
        match chars.peek() {
            Some(c) => {
                if c.is_digit(10) {
                    number.push(chars.next().unwrap());
                    optional = Some(number);
                } else {
                    return Some(number);
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
