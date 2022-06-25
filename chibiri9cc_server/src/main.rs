use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};

use chibiri9cc_lib::{gen, parser, tokenize};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello compiler api\n")
}

#[post("/compile")]
async fn compile(req_body: String) -> impl Responder {
    let token = tokenize::tokenize(std::rc::Rc::new(req_body), 0);
    if let Err(err) = token {
        return HttpResponse::BadRequest().body(format!("{}", err));
    }

    let node = parser::expr(&mut token.unwrap());
    if let Err(err) = node {
        return HttpResponse::BadRequest().body(format!("{}", err));
    }

    let mut generated = String::new();
    generated += "  .globl main\n";
    generated += "main:\n";

    generated += &gen::gen(node.unwrap()).unwrap();

    generated += "  ret\n";
    HttpResponse::Ok().body(generated)
}

#[actix_web::main]
async fn main() -> std::result::Result<(), std::io::Error> {
    HttpServer::new(|| App::new().service(compile).service(hello))
        .bind(("0.0.0.0", 3000))?
        .run()
        .await
}
