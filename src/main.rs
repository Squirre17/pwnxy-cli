use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use globalenv::{set_var, unset_var};
use base64::{Engine as _, engine::general_purpose};
use std::env;


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    
    // println!("{}", req_body.as_str());
    let data = general_purpose::STANDARD
                               .decode(req_body)
                               .unwrap();
    println!("{}", String::from_utf8(data).unwrap());

    HttpResponse::Ok()
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    // println!("{:?}", env::args());
    let mut argv : Vec<String> = env::args().collect();
    let argn = argv.len();
    let name;

    if argn == 2 {
        name = argv.pop();
    }
    
    

    set_var("PWNXY_CLIENT_PORT", "8080").unwrap();

    let srv = HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 0))?;

    println!("{:#?}", srv.addrs()[0].port());

    srv.run().await
}