use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::fs;

#[get("/getPubKey")]
async fn ping() -> HttpResponse {
    println!("recieved request");
    let data = fs::read("../../example/1/pub.toml").expect("Unable to read file");
    HttpResponse::Ok()
    .body(data)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let port = &args[1];

    println!("running reciever on port: {}", port);
    
    HttpServer::new(|| {
        App::new()
            .service(ping)
    })
    .bind(("0.0.0.0", port.parse::<u16>().unwrap()))?
    .run()
    .await
}
