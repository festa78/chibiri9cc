mod compile;
mod tokenize;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Invalid number fo arguments");
        std::process::exit(1);
    }

    let statement = args[1].to_string();

    compile::compile(tokenize::tokenize(&mut statement.chars().peekable()).unwrap()).unwrap()
}
