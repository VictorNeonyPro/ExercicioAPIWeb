use std::ops::Deref;
use actix_web::{get, post, web, HttpResponse, Responder};
use crate::AppState;
use crate::models::user::User;

#[get("/user")]
pub async fn get_user(data: web::Data<AppState>) -> impl Responder {
    let user = data.user.lock().unwrap();

    let message = match user.deref() {
        Some(user) => user.to_string(),
        _ => "No User Registered".to_string()
    };

    HttpResponse::Ok().body(message)
}

#[post("/user")]
pub async fn set_user(data: web::Data<AppState>, json: web::Json<User>) -> impl Responder {
    let mut user = data.user.lock().unwrap();
    *user = Some(json.into_inner());
    HttpResponse::Created().body("User registered")
}