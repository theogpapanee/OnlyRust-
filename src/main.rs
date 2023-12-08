

use actix_files::Files;
use actix_web::web::Bytes;
use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};
use rusoto_core::{Region, RusotoError};
use rusoto_credential::{StaticProvider, ProvideAwsCredentials};
use rusoto_s3::{PutObjectRequest, S3, S3Client};
use std::env;
use chrono::{Utc, DateTime};


use actix_web::{web, HttpResponse, Responder, HttpServer, App, Result};
use rusqlite::{Connection, Error as RusqliteError};
use rusqlite::params;

#[derive(Debug)]
struct User {
    email: String,
    username: String,
    hashed_password: String,
}
//INCLUDING THE HTML BASICALL
async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../static/index.html")) // Path 
}

async fn create_account(data: web::Form<FormData>) -> Result<HttpResponse> {
    // EXTRACTING DATA FROM THE CREATE ACCOUNT PAGE
    let email = &data.email;
    let username = &data.username;
    let hashed_password = &data.password; // Inputs 

    // OPPERATIONS
    match insert_user(&email, &username, &hashed_password) {
        Ok(_) => Ok(HttpResponse::Ok().body("Account created successfully")),
        Err(e) => {
            eprintln!("Failed to insert user: {:?}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}
//INSERTING THE LIST OF PEOPLE WHO ARE SUBBED
pub fn insert_subscription(conn: &Connection, user_id: i64, user_unlocked: i64, plan_name: &str, start_date: DateTime<Utc>, price: f32) -> rusqlite::Result<i64> {
    let end_date: DateTime<Utc> = end_date_calculator(&plan_name, start_date);
    let mut stmt = conn.prepare("INSERT INTO subscriptions (user_id, plan_name, user_unlocked, start_date, end_date) VALUES (?1, ?2, ?3, ?4, ?5)")?;
    stmt.execute(params![user_id, plan_name, user_unlocked, start_date.to_rfc3339(), end_date.to_rfc3339()])?;
    Ok(conn.last_insert_rowid())
}
//CHECKS IF USER EXISTS AND RETURNS THEIR ID

pub fn get_user_id_by_username(conn: &Connection, username: &str) -> rusqlite::Result<Option<i64>> {
    let mut stmt = conn.prepare("SELECT id FROM users WHERE username = ?1")?;
    let mut rows = stmt.query(params![username])?;

    if let Some(row) = rows.next()? {
        Ok(Some(row.get(0)?))
    } else {
        Ok(None)
    }
}

//HOW YOU INSERT USERS
fn insert_user(email: &str, username: &str, hashed_password: &str) -> Result<(), RusqliteError> {
    let conn = Connection::open("test.db")?;
    create_tables(&conn)?;

    conn.execute(
        "INSERT INTO users (email, username, hashed_password) VALUES (?1, ?2, ?3)",
        &[email, username, hashed_password],
    )?;
    Ok(())
}

fn create_tables(conn: &Connection) -> Result<(), RusqliteError> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY,
                email TEXT NOT NULL,
                username TEXT NOT NULL,
                hashed_password TEXT NOT NULL
            )",
        [],
    )?;
    Ok(())
}

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    username: String,
    password: String,
}

async fn search_user(username: web::Query<Username>) -> impl Responder {
    let conn = Connection::open("test.db").expect("Failed to open database");
    
    match get_user_id_by_username(&conn, &username.username) {
        Ok(Some(_)) => HttpResponse::Ok().body("User Found!"),
        Ok(None) => HttpResponse::NotFound().body("User not Found"),
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}

fn end_date_calculator(plan_name: &str, start_date: DateTime<Utc>) -> DateTime<Utc>{
    match plan_name {
        "plan_a" => start_date + chrono::Duration::days(30),
        "plan_b" => start_date + chrono::Duration::days(60), 
        _ => Utc::now(),
    }
}

#[derive(serde::Deserialize)]
struct Username {
    username: String,
}

//BASICALLy RUNS THE WHOLE THING AND HELPS CREATE ACCOUNT
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/create_account", web::post().to(create_account)) 
            .service(actix_files::Files::new("/static", "static").show_files_listing())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}


//NEW CODE THAT MIGHT WORK BETTER IF IMPLEMENTED

/*

use actix_web::{web, App, HttpServer, HttpResponse, Result};
use actix_web::web::Bytes;
use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};
use rusoto_core::{Region, RusotoError};
use rusoto_credential::{StaticProvider, ProvideAwsCredentials};
use rusoto_s3::{PutObjectRequest, S3, S3Client};
use std::env;

async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../static/index.html"))
}

async fn upload_file(mut payload: Multipart) -> Result<HttpResponse> {
    while let Some(field) = payload.next().await {
        let mut field = field?;
        let content_type = field.content_disposition().ok_or_else(|| HttpResponse::BadRequest())?;
        let filename = content_type.get_filename().unwrap_or("unnamed.png");

        let mut data = Bytes::new();
        while let Some(chunk) = field.next().await {
            let chunk = chunk?;
            data.extend_from_slice(chunk.as_ref());
        }

        let access_key = env::var("AWS_ACCESS_KEY_ID").expect("AWS_ACCESS_KEY_ID not found");
        let secret_key = env::var("AWS_SECRET_ACCESS_KEY").expect("AWS_SECRET_ACCESS_KEY not found");
        let region = Region::UsEast1;

        let provider = StaticProvider::new_minimal(access_key, secret_key);
        let client = S3Client::new_with(
            rusoto_core::request::HttpClient::new().unwrap(),
            provider,
            region,
        );

        upload_to_s3(&client, &data, filename, "your_bucket_name").await?;

        return Ok(HttpResponse::Ok().into());
    }

    Ok(HttpResponse::BadRequest().into())
}

async fn upload_to_s3(
    client: &S3Client,
    data: &[u8],
    filename: &str,
    bucket_name: &str,
) -> Result<(), RusotoError<rusoto_s3::PutObjectError>> {
    let request = PutObjectRequest {
        bucket: bucket_name.to_owned(),
        key: filename.to_owned(),
        body: Some(data.into()),
        ..Default::default()
    };

    client.put_object(request).await.map(|_| ())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/upload", web::post().to(upload_file))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
 */
/*use actix_files::Files;
[dependencies]
actix-web = "4.4.0"
actix-files = "0.6.2" */


