use actix_web::{post, get, web, App, HttpResponse, HttpServer, HttpRequest};
use crate::web::Bytes;
use std::{fs, io::Write};
use std::fs::OpenOptions;


#[post("/postData/{lightclientNumber}")]
async fn post_data(path: web::Path<String>, body: Bytes) -> HttpResponse {
    let lightclient_number: i32  = path.into_inner().as_str().parse().unwrap();
    println!("received data from lightclient {}", lightclient_number + 1);
    let data_file = format!("../../test_data/data{}.csv", lightclient_number);
    let string_body = String::from_utf8(body.to_vec());
    fs::write(data_file, string_body.unwrap().as_str()).expect("Unable to write file");

    let light_client_number_formatted = format!("{},",lightclient_number);
    
    let lightclient_numbers_file = "../lightclient_numbers/light_clients_on_network.txt";
    let light_client_numbers_from_file = fs::read_to_string(lightclient_numbers_file).expect("Unable to read file");

    let light_client_numbers_string = light_client_numbers_from_file + &light_client_number_formatted;
    
    let file_write_response = fs::write(lightclient_numbers_file, light_client_numbers_string);
    match file_write_response {
        Ok(file) => file,
        Err(error) => panic!("Error writing to file: {:?}", error),
    }

    let http_response_message = format!("succesfully saved data file for lightclient {}", lightclient_number + 1);
    HttpResponse::Ok().body(http_response_message)
}

#[get("/dataFilledConfirm/{lightclientNumber}")]
async fn data_filled_confirm(path: web::Path<String>) -> HttpResponse {
    let lightclient_number: i32  = path.into_inner().as_str().parse().unwrap();
    for i in 0..lightclient_number {
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
    let args: Vec<String> = std::env::args().collect();
    let port = &args[1];

    println!("running data_reciever at 0.0.0.0:{}", port);
    HttpServer::new(|| {
        App::new()
            .service(post_data)
            .service(data_filled_confirm)
    })
    .bind(("0.0.0.0", port.parse::<u16>().unwrap()))?
    .run()
    .await
}
