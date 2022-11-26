use actix_web::{post, web, App, HttpResponse, HttpServer, HttpRequest};
use crate::web::Bytes;
use std::fs;


#[post("/postData/{lightclientNumber}")]
async fn post_data(path: web::Path<String>, req: HttpRequest, body: Bytes) -> HttpResponse {
    let lightclient_number: i32  = path.into_inner().as_str().parse().unwrap();
    println!("received data from lightclient {}", lightclient_number + 1);
    let data_file = format!("../../test_data/data{}.csv", lightclient_number);
    let string_body = String::from_utf8(body.to_vec());
    println!("BODY: {:#?}", string_body);
    fs::write(data_file, string_body.unwrap().as_str()).expect("Unable to write file");

    let http_response_message = format!("succesfully saved data file for lightclient {}", lightclient_number + 1);
    HttpResponse::Ok().body(http_response_message)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("starting data_reciever at 0.0.0.0:8000");
    HttpServer::new(|| {
        App::new()
            .service(post_data)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
