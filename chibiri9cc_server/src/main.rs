use actix_web::{post, App, HttpResponse, HttpServer, Responder};

// use chibiri9cc_lib::{gen, parser, tokenize};

#[post("/")]
async fn compile(req_body: String) -> impl Responder {
  HttpResponse::Ok().body(req_body)
  // let statement = args[1].to_string();
  // let token = tokenize::tokenize(std::rc::Rc::new(statement), 0);
  // if let Err(err) = token {
  //   eprint!("{}", err);
  //   std::process::exit(1);
  // }

  // let node = parser::expr(&mut token.unwrap());
  // if let Err(err) = node {
  //   eprint!("{}", err);
  //   std::process::exit(1);
  // }

  // // println!(".intel_syntax noprefix");
  // println!("  .globl main");
  // println!("main:");

  // if let Err(err) = gen::gen(node.unwrap()) {
  //   eprint!("{}", err);
  //   std::process::exit(1);
  // }

  // println!("  ret");
}

#[actix_web::main]
async fn main() -> std::result::Result<(), std::io::Error> {
  HttpServer::new(|| App::new().service(compile))
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
