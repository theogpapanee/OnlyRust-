use rusqlite::{Connection, Result,NO_PARAMS};
use chrono::{Utc, DateTime};
use rusoto_core::{Region, RusotoError};
use rusoto_s3::{PutObjectRequest, S3, S3Client};
use crate::accounts::{User, Subscription, FileRecord};

pub fn create_tables(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
             id INTEGER PRIMARY KEY,
             email TEXT not null unique,
             username TEXT not null unique,
             password_hash TEXT
         )",
        NO_PARAMS,
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS subscriptions (
             id INTEGER PRIMARY KEY,
             user_id INTEGER,
             user_unlocked INTEGER,
             plan_name TEXT not null,
             start_date TEXT,
             end_date TEXT,
             FOREIGN KEY (user_id) REFERENCES users(id)
         )",
        NO_PARAMS,
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS files(
            id INTEGER PRIMARY KEY,
            user_id INTEGER,
            file_name TEXT unique,
            file_description TEXT,
            s3_object_key TEXT,
            upload_date TEXT,
            FOREIGN KEY (user_id) REFERENCES users(id)
        )",
        NO_PARAMS
    )?;

    Ok(())
}

pub fn insert_user(conn: &Connection, email: &str, username: &str, password_hash: &str) -> rusqlite::Result<i64> {
    let hashed_pass: &str = hash_password(password_hash).as_str();
    let mut stmt = conn.prepare("INSERT INTO users (email, username, hashed_pass) VALUES (?1, ?2, ?3)")?;
    stmt.execute(params![email, username, hashed_pass])?;
    Ok(conn.last_insert_rowid())
}

fn end_date_calculator(plan_name: &str, start_date: DateTime<Utc>) -> Result<DateTime<Utc>>{
    match plan_name {
        "plan_a" => start_date + chrono::Duration::days(30),
        "plan_b" => start_date + chrono::Duration::days(60), 
        _ => Err()
    }
}

pub fn insert_subscription(conn: &Connection, user_id: i64, user_unlocked: i64, plan_name: &str, start_date: DateTime<Utc>) -> rusqlite::Result<i64> {
    let end_date: DateTime<Utc> = end_date_calculator(&plan_name, start_date);
    let mut stmt = conn.prepare("INSERT INTO subscriptions (user_id, plan_name, user_unlocked, start_date, end_date) VALUES (?1, ?2, ?3, ?4, ?5)")?;
    stmt.execute(params![user_id, plan_name, user_unlocked, start_date.to_rfc3339(), end_date.to_rfc3339()])?;
    Ok(conn.last_insert_rowid())
}

pub fn insert_file_to_database(conn: &Connection, user_id: i64, file_name: &str, file_description: &str, s3_object_key: &str, upload_date: DateTime<Utc>) -> rusqlite::Result<i64> {
    let mut stmt = conn.prepare("INSERT INTO files (user_id, file_name, file_description, s3_object_key, upload_date) VALUES (?1, ?2, ?3, ?4, ?5)")?;
    stmt.execute(params![user_id, file_name, file_description, s3_object_key, upload_date.to_rfc3339()])?;
    Ok(conn.last_insert_rowid())
}

pub fn insert_file_s3_and_database(conn: &Connection, user_id: i64, file_name: &str, file_description: &str, s3_client: &S3Client, bucket_name: &str) -> Result<>{
    let s3_object_key = format!("uploads/{}", file_name);
    let upload_date = Utc::now();
    let file_id = insert_file_to_database(&conn, user_id, file_name, file_description, &s3_object_key, upload_date).expect("Failed to insert file");
    insert_file_to_s3(&s3_client, bucket_name, &s3_object_key, file_content).expect("Failed to upload file to S3");
}

pub fn get_user_id_by_email(conn: &Connection, email: &str) -> rusqlite::Result<Option<i64>> {
    let mut stmt = conn.prepare("SELECT id FROM users WHERE email = ?1")?;
    let mut rows = stmt.query(params![username])?;

    if let Some(row) = rows.next()? {
        Ok(Some(row.get(0)?))
    } else {
        Ok(None)
    }
}

pub fn get_files_by_user_id(conn: &Connection, user_id: i64) -> rusqlite::Result<Vec<FileRecord>> {
    let mut stmt = conn.prepare("SELECT id, user_id, file_name, s3_object_key, upload_date FROM files WHERE user_id = ?1")?;
    let rows = stmt.query_map(params![user_id], |row| {
        Ok(FileRecord {id: row.get(0)?,user_id: row.get(1)?,file_name: row.get(2)?,s3_object_key: row.get(3)?,upload_date: row.get(4)?,})
    })?;

    rows.collect()
}

pub fn is_subscribed(conn: &Connection, user_id: i64, user_unlocked:i64) -> rusqlite::Result<bool>{
    let mut stmt = conn.prepare("SELECT id, user_id, user_unlocked, plan_name, start_date, end_date FROM subscriptions WHERE user_id = ?1 AND user_unlocked = ?2")?;
    let rows = stmt.query_map(params![user_id, user_unlocked], |row| {
        Ok(Subscription {
            id: row.get(0)?,
            user_id: row.get(1)?,
            user_unlocked: row.get(2)?,
            plan_name: row.get(3)?,
            start_date: DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)?.with_timezone(&Utc),
            end_date: DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)?.with_timezone(&Utc),
        })
    })?;
    rows = rows.collect();
    //returns vector of Subscription structs
    for subscriptions in rows{
        let now: DateTime<Utc> = Utc::now();
        if(now >= subscription.start_date && now <= subscription.end_date){
            return true;
        }
    }
    false
}