use actix_web::{guard, web, HttpResponse};

use crate::handlers::handlers::index;

pub fn config_rlo_scope() -> actix_web::Scope {
    web::scope("/")
        .guard(guard::Host("www.rust-lang.org"))
        .route("", web::to(|| async { 
            HttpResponse::Ok().body("www") 
        }))
}

pub fn config_urlo_scope() -> actix_web::Scope {
    web::scope("/")
    .guard(guard::Host("users.rust-lang.org"))
    .route("", web::to(|| async { 
        HttpResponse::Ok().body("user") 
    }))
}

pub fn config_index_scope() ->actix_web::Scope {
    web::scope("/app")
    .route("/index.html", web::get()
    .to(index))
}