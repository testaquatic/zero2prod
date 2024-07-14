use actix_web::{HttpResponse, Responder};

pub async fn greet() -> impl Responder {
    let body = "Hello World!".to_string();
    HttpResponse::Ok().body(body)
}
