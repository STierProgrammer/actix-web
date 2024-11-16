use actix_web::{web, HttpResponse, Responder};

use crate::models::states::AppStateWithCounter;

pub async fn mutex_state(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;

    format!("Request number: {counter}")
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

pub async fn index() -> impl Responder {
    "Hello World!"
}
