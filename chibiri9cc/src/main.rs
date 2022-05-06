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
    match tokenize::tokenize(&statement, 0) {
        Ok(token) => match compile::compile(token) {
            Ok(()) => (),
            Err(err) => eprintln!("{}", err),
        },
        Err(err) => eprintln!("{}", err),
    }
}
