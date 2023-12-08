use sqlx::FromRow;
use chrono::{Utc, DateTime};
use argon2::Error;
use sqlx::Row;

#[derive(FromRow)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub balance: f64,
}
// 
#[derive(Debug)]
pub struct Subscription {
    id: i64,
    user_id: i64,
    user_unlocked: i64,
    plan_name: String,
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
    subscription_price: f64
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

/*
impl Subscription {
    pub fn new(id: i64, user_id: i64, user_unlocked: i64, plan_name: String, start_date: DateTime<Utc>, end_date: DateTime<Utc>, subscription_price: f64,) -> Self {
        Subscription {id,user_id,user_unlocked,plan_name,start_date,end_date,subscription_price,}
    }

    pub fn from_row(row: &rusqlite::Row<'_>) -> Result<Subscription, Error> {
        Ok(Subscription::new(
            row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)?.with_timezone(&Utc), DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)?.with_timezone(&Utc),row.get(6)?,
        ))
    }
}
*/

/*
impl FileRecord {
    // Private constructor
    pub fn new(id: i64,user_id: i64,file_name: String,file_description: String,s3_object_key: String,upload_date: DateTime<Utc>,) -> Self {
        FileRecord {id,user_id,file_name,file_description,s3_object_key,upload_date,}
    }
    pub fn from_row(row: &impl Row) -> Result<FileRecord, Error> {
        Ok(FileRecord::new(row.get(0)?,row.get(1)?,row.get(2)?,row.get(3)?,row.get(4)?,
            DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)?.with_timezone(&Utc),
        ))
    }
}
*/