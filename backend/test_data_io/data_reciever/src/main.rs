use actix_web::{post, get, web, App, HttpResponse, HttpServer, HttpRequest};
use crate::web::Bytes;
use std::fs;


#[post("/postData/{lightclientNumber}")]
async fn post_data(path: web::Path<String>, req: HttpRequest, body: Bytes) -> HttpResponse {
    let lightclient_number: i32  = path.into_inner().as_str().parse().unwrap();
    println!("received data from lightclient {}", lightclient_number + 1);
    let data_file = format!("../../test_data/data{}.csv", lightclient_number);
    let string_body = String::from_utf8(body.to_vec());
    fs::write(data_file, string_body.unwrap().as_str()).expect("Unable to write file");

    let http_response_message = format!("succesfully saved data file for lightclient {}", lightclient_number + 1);
    HttpResponse::Ok().body(http_response_message)
}

#[get("/dataFilledConfirm")]
async fn data_filled_confirm() -> HttpResponse {
    for i in 0..10 {
        let data_file = format!("../../test_data/data{}.csv",i);
        let data = fs::read(data_file).expect("Unable to read file");
        if data.is_empty() {
            let error_message = format!("test data for lightclient {} not present", i + 1);
            return HttpResponse::InternalServerError().body(error_message)
        }
    }
    HttpResponse::Ok().body("test data for all lightclients present")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("starting data_reciever at 0.0.0.0:8000");
    HttpServer::new(|| {
        App::new()
            .service(post_data)
            .service(data_filled_confirm)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
