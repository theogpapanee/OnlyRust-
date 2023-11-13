use sqlx::FromRow;

#[derive(FromRow)]
pub struct Account {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
}