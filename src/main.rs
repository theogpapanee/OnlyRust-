use sqlx::PgPool;
use sqlx::postgres::PgConnectOptions;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_url = "link";
    let db_pool = configure_database(db_url);

    //implment more startup thingy

    actix_web::HttpServer::new(move || {
        App::new().data(db_pool.clone())
        //implement more stuff
    }).bind("localip").run()
}

async fn configure_database(db_url: &str) -> Result<PgPool, sqlx::Error> {
    let db_pool = PgPool::connect_with(PgConnectOptions::new().connect_timeout(std::time::Duration::from_secs(10)).username("user").password("pass").database("db").build());
    Ok(db_pool)
}