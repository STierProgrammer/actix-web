pub mod models;
pub mod handlers;
pub mod routes;

use std::sync::Mutex;

use actix_web::{web, App, HttpResponse, HttpServer};
use handlers::handlers::{manual_hello, mutex_state};
use models::states::{AppState, AppStateWithCounter};
use routes::{routes::{echo, hello, state}, scopes::{config_index_scope, config_rlo_scope, config_urlo_scope}};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0)
    });
    
    HttpServer::new(move || {
        App::new()
            .service(config_rlo_scope())
            .service(config_urlo_scope())
            .service(config_index_scope())
            .service(hello)
            .service(echo)
            .service(state)
            
            .route("/hey", web::get().to(manual_hello))
            .route("/counter", web::get().to(mutex_state))
            .route("/", web::to(HttpResponse::Ok))
            
            .app_data(counter.clone())
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix Web")
            }))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}