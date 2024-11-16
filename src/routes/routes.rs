use actix_web::{get, post, web, HttpResponse, Responder};

use crate::models::states::AppState;

#[get("/index")]
pub async fn state(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;

    format!("Hello {app_name}!")
}

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}