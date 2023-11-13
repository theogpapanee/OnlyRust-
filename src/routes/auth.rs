//authenticate


use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::user::User;
use crate::password::{hash_password, verify_password};

async fn create_account(
    pool: web::Data<PgPool>,
    info: web::Json<CreateAccountInfo>,
) -> impl Responder {
    let hashed_password = hash_password(&info.password);

    let result = sqlx::query!(
        "type name, email, and pass",
        info.username,
        info.email,
        hashed_password
    ).fetch_one(pool.as_ref()).await;

    match result {
        Ok(user) => HttpResponse::Created().json(user.id),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[derive(serde::Deserialize)]
struct CreateAccountInfo {
    username: String,
    email: String,
    password: String,
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/create-account").route(web::post().to(create_account)));
}
