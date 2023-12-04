use sqlx::FromRow;
use chrono::{Utc, DateTime};

#[derive(FromRow)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub password_hash: String,
}

#[derive(Debug)]
pub struct Subscription {
    id: i64,
    user_id: i64,
    user_unlocked: i64,
    plan_name: String,
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
}

#[derive(Debug)]
pub struct FileRecord {
    id: i64,
    user_id: i64,
    file_name: String,
    file_description: String,
    s3_object_key: String, //cloud storage
    upload_date: DateTime<Utc>,
}

