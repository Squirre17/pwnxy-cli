use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use globalenv::{set_var, unset_var};
use base64::{Engine as _, engine::general_purpose};
use std::env;
use std::fs;
use std::os;
use std::io;
use std::path::{Path, PathBuf};
use std::io::Write;
use rand::{distributions::Alphanumeric, Rng}; // 0.8
use nix::sys::signal;
use std::{error::Error};
// use signal_hook::{iterator::Signals, SIGTERM};
use std::{thread, time::Duration};


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


fn write_to_file(name : String, port : & u16) -> io::Result<PathBuf> {

    let dirname = Path::new("./pwnxy_cli_sync");

    dirname.is_dir().then(||{}).or_else(|| {
        fs::create_dir(dirname).and_then(|_| {
            println!("{} created successfully", dirname.to_str().unwrap());
            Ok(0)
        }).or_else(|_| {
            println!("Can't create {} ", dirname.to_str().unwrap());
            Err(0)
        }).unwrap_or_else(|_| {0});
        Some(())
    });

    let path = dirname.join(name);
    let mut f = std::fs::File::create(&path)?;
    f.write_all(port.to_string().as_bytes())?;

    Ok(path)
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let mut argv : Vec<String> = env::args().collect();
    let argn = argv.len();
    let name;

    if argn == 2 {
        name = argv.pop().unwrap();
        // println!("dbg {}", name.as_str());
    }else {
        // TODO: check for name collision
        name = rand::thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(4)
                    .map(char::from)
                    .collect();
    }

    let srv = HttpServer::new(|| {
        App::new()
            .service(echo)
            // .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 0))?;

    let port = srv.addrs()[0].port();
    let path : PathBuf = write_to_file(name, &port).unwrap();
    println!("[NOTE] : client run on port : {}", port);

    
    // TODO: exit action register
    ctrlc::set_handler(move || {
        fs::remove_file(&path).unwrap();
    }).expect("Error setting Ctrl-C handler");

    srv.run().await
}