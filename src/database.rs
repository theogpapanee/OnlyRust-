use rusqlite::{Connection, Result,params};
use chrono::{Utc, DateTime};
use rusoto_core::{Region, RusotoError};
use rusoto_s3::{PutObjectRequest, S3, S3Client};
use crate::account::{User, Subscription, FileRecord};
use crate::s3::insert_file_to_s3;
use crate::passwordhash::verify_password;
use argon2::Error;

pub fn create_tables(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
             id INTEGER PRIMARY KEY,
             email TEXT not null unique,
             username TEXT not null unique,
             password_hash TEXT,
             balance REAL DEFAULT 0.0
         )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS subscriptions (
             id INTEGER PRIMARY KEY,
             user_id INTEGER,
             user_unlocked INTEGER,
             plan_name TEXT not null,
             start_date TEXT,
             end_date TEXT,
             price REAL,
             FOREIGN KEY (user_id) REFERENCES users(id)
         )",
        [],
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
        []
    )?;

    Ok(())
}

pub fn insert_user(conn: &Connection, email: &str, username: &str, password_hash: &str) -> rusqlite::Result<i64> {
    //let hashed_pass: &str = hash_password(password_hash).as_str();
    //let mut stmt = conn.prepare("INSERT INTO users (email, username, hashed_pass) VALUES (?1, ?2, ?3)")?;
    let mut stmt = conn.prepare("INSERT INTO users (email, username, password_hash) VALUES (?1, ?2, ?3)")?;
    stmt.execute(params![email, username, password_hash])?;
    Ok(conn.last_insert_rowid())
}

pub fn insert_subscription(conn: &Connection, user_id: i64, user_unlocked: i64, plan_name: &str, start_date: DateTime<Utc>, price: f32) -> rusqlite::Result<i64> {
    let end_date: DateTime<Utc> = end_date_calculator(&plan_name, start_date);
    let mut stmt = conn.prepare("INSERT INTO subscriptions (user_id, plan_name, user_unlocked, start_date, end_date) VALUES (?1, ?2, ?3, ?4, ?5)")?;
    stmt.execute(params![user_id, plan_name, user_unlocked, start_date.to_rfc3339(), end_date.to_rfc3339()])?;
    Ok(conn.last_insert_rowid())
}

fn end_date_calculator(plan_name: &str, start_date: DateTime<Utc>) -> DateTime<Utc>{
    match plan_name {
        "plan_a" => start_date + chrono::Duration::days(30),
        "plan_b" => start_date + chrono::Duration::days(60), 
        _ => Utc::now(),
    }
}

pub fn is_subscribed(conn: &Connection, subscriber_id: i64, user_unlocked: i64) -> rusqlite::Result<bool> {
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM subscriptions WHERE user_id = ?1 AND user_unlocked = ?2 AND end_date > CURRENT_TIMESTAMP")?;
    let count: i64 = stmt.query_row(params![subscriber_id, user_unlocked], |row| row.get(0))?;

    // If the count is greater than 0, it means the user is subscribed
    Ok(count > 0)
}

pub fn verify_password_by_email(conn: &Connection, email: &str, provided_password: &str) -> rusqlite::Result<bool> {
    let mut stmt = conn.prepare("SELECT password_hash FROM users WHERE email = ?1")?;
    let mut rows = stmt.query(params![email])?;

    if let Some(row) = rows.next()? {
        let stored_password_hash: String = row.get(0)?;
        Ok(verify_password(&stored_password_hash, provided_password))
    } else {
        Ok(false)
    }
}

pub fn update_user_balance(conn: &Connection, user_id: i64, new_balance: f64) -> rusqlite::Result<()> {
    conn.execute("UPDATE users SET balance = ?1 WHERE id = ?2", params![new_balance, user_id])?;
    Ok(())
}

pub fn get_user_id_by_username(conn: &Connection, username: &str) -> rusqlite::Result<Option<i64>> {
    let mut stmt = conn.prepare("SELECT id FROM users WHERE username = ?1")?;
    let mut rows = stmt.query(params![username])?;

    if let Some(row) = rows.next()? {
        Ok(Some(row.get(0)?))
    } else {
        Ok(None)
    }
}

pub fn get_user_balance(conn: &Connection, user_id: i64) -> rusqlite::Result<f64> {
    let mut stmt = conn.prepare("SELECT balance FROM users WHERE id = ?1")?;
    let mut rows = stmt.query(params![user_id])?;

    if let Some(row) = rows.next()? {
        Ok(row.get(0)?)
    } else {
        Err(rusqlite::Error::QueryReturnedNoRows)
    }
}

pub fn is_broke(conn: &Connection, user_id: i64, price: f64) -> Result<bool, rusqlite::Error> {
    let balance_result: Result<f64, rusqlite::Error> = get_user_balance(conn, user_id);
    let result: bool = match balance_result {
        Ok(balance) => balance < price,
        Err(err) => false
    };
    Ok(result)
}
/*

pub fn insert_file_to_database(conn: &Connection, user_id: i64, file_name: &str, file_description: &str, s3_object_key: &str, upload_date: DateTime<Utc>) -> rusqlite::Result<i64> {
    let mut stmt = conn.prepare("INSERT INTO files (user_id, file_name, file_description, s3_object_key, upload_date) VALUES (?1, ?2, ?3, ?4, ?5)")?;
    stmt.execute(params![user_id, file_name, file_description, s3_object_key, upload_date.to_rfc3339()])?;
    Ok(conn.last_insert_rowid())
}

pub fn insert_file_s3_and_database(conn: &Connection, user_id: i64, file_name: &str, file_description: &str, s3_client: &S3Client, file_content: Vec<u8>, bucket_name: &str) -> Result<i32>{
    let s3_object_key = format!("uploads/{}", file_name);
    let upload_date = Utc::now();
    let file_id = insert_file_to_database(&conn, user_id, file_name, file_description, &s3_object_key, upload_date).expect("Failed to insert file");
    insert_file_to_s3(&s3_client, bucket_name, &s3_object_key, file_content);
    Ok(1)
}

pub fn get_user_id_by_email(conn: &Connection, email: &str) -> rusqlite::Result<Option<i64>> {
    let mut stmt = conn.prepare("SELECT id FROM users WHERE email = ?1")?;
    let mut rows = stmt.query(params![email])?;

    if let Some(row) = rows.next()? {
        Ok(Some(row.get(0)?))
    } else {
        Ok(None)
    }
}

//broken
/*
pub fn get_files_by_user_id(conn: &Connection, user_id: i64) -> rusqlite::Result<Vec<FileRecord>> {
    let mut stmt = conn.prepare("SELECT id, user_id, file_name, s3_object_key, upload_date FROM files WHERE user_id = ?1")?;
    let rows = stmt.query_map(params![user_id], |row| {
        Ok(FileRecord::new(row.get(0)?,row.get(1)?,row.get(2)?,row.get(3)?,row.get(4)?,row.get(5)?,))
    })?;

    rows.collect()
}
*/


pub fn is_subscribed(conn: &Connection, user_id: i64, user_unlocked: i64) -> Result<bool, Error> {
    let sql = "SELECT id, user_id, user_unlocked, plan_name, start_date, end_date, subscription_price FROM subscriptions WHERE user_id = ?1 AND user_unlocked = ?2";
    
    let mut stmt = conn.prepare(sql)?;

    let now: DateTime<Utc> = Utc::now();

    let is_subscribed = stmt.query_and_then(params![user_id, user_unlocked], |row| {
        let subscription = Subscription::from_row(row)?;
        Ok(now >= subscription.start_date && now <= subscription.end_date)
    })?.any(Result::unwrap_or_default);

    Ok(is_subscribed)
}


pub fn get_user_balance(conn: &Connection, user_id: i64) -> rusqlite::Result<f64> {
    let mut stmt = conn.prepare("SELECT balance FROM users WHERE id = ?1")?;
    let mut rows = stmt.query(params![user_id])?;

    if let Some(row) = rows.next()? {
        Ok(row.get(0)?)
    } else {
        Err(rusqlite::Error::QueryReturnedNoRows)
    }
}

pub fn is_broke(conn: &Connection, user_id: i64, price: f64) -> Result<bool, rusqlite::Error> {
    let balance_result: Result<f64, rusqlite::Error> = get_user_balance(conn, user_id);
    let result: bool = match balance_result {
        Ok(balance) => balance < price,
        Err(err) => false
    };
    Ok(result)
}



*/
