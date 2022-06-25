use chibiri9cc_lib::{gen, parser, tokenize};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Invalid number of arguments");
        std::process::exit(1);
    }

    let statement = args[1].to_string();
    let token = tokenize::tokenize(std::rc::Rc::new(statement), 0);
    if let Err(err) = token {
        eprint!("{}", err);
        std::process::exit(1);
    }

    let node = parser::expr(&mut token.unwrap());
    if let Err(err) = node {
        eprint!("{}", err);
        std::process::exit(1);
    }

    let mut generated = String::new();
    generated += "  .globl main\n";
    generated += "main:\n";

    generated += &gen::gen(node.unwrap()).unwrap();

    generated += "  ret\n";

    println!("{}", generated);
}
