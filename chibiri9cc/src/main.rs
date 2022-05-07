mod compile;
mod statement;
mod tokenize;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Invalid number fo arguments");
        std::process::exit(1);
    }

    let statement = args[1].to_string();
    let token = tokenize::tokenize(std::rc::Rc::new(statement), 0);
    if let Err(err) = token {
        eprint!("{}", err);
        std::process::exit(1);
    }
    if let Err(err) = compile::compile(token.unwrap()) {
        eprint!("{}", err);
        std::process::exit(1);
    }
}
