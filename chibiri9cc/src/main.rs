mod tokenize;

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
    let token = tokenize::tokenize(&mut chars).unwrap();
    if token.kind == tokenize::TokenKind::Eof {
        panic!("no token found");
    }

    println!("  mov rax, {}", &token.str.expect("token should have str"));

    let mut next_ops_token = *token.next.expect("Invalide statement");

    while next_ops_token.kind != tokenize::TokenKind::Eof {
        if next_ops_token.kind != tokenize::TokenKind::Reserved {
            panic!(
                "unknown token {}, expect ops",
                next_ops_token.str.expect("token should have str")
            );
        }

        let next_num_token = *next_ops_token.next.expect("Invalid statement");
        if next_num_token.kind != tokenize::TokenKind::Num {
            panic!(
                "unknown token {}, expect number",
                next_num_token.str.expect("token should have str")
            );
        }

        match next_ops_token
            .str
            .expect("token should have str")
            .chars()
            .next()
            .unwrap()
        {
            '+' => println!(
                "  add rax, {}",
                next_num_token.str.expect("token should have str")
            ),
            '-' => println!(
                "  sub rax, {}",
                next_num_token.str.expect("token should have str")
            ),
            _ => panic!(
                "unsupported ops {}",
                next_num_token.str.expect("token should have str")
            ),
        }

        next_ops_token = *next_num_token.next.expect("Invalid statement");
    }

    println!("  ret");
}
