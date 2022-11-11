use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::fs;

// fn getFileBytes() -> Result<Vec<u8>> {
//     String::from_utf8_lossy(&fs::read("address.txt")?).parse()?;
// }

#[get("/getPubKey")]
async fn ping() -> HttpResponse {
    let data = fs::read("../../example/1/pub.toml").expect("Unable to read file");
    HttpResponse::Ok()
    .body(data)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(ping)
    })
    .bind(("0.0.0.0", 6335))?
    .run()
    .await
}
