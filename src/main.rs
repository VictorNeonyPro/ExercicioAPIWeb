mod api;
mod models;

use std::sync::Mutex;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use crate::api::user::{get_user, set_user};
use crate::models::user::User;

struct AppState {
    user: Mutex<Option<User>>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        user: Mutex::new(None)
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(get_user)
            .service(set_user)
    })
    .bind("127.0.0.1:8080")?.run().await
}
