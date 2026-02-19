use sqlx::PgPool;
use std::io::Error as IoError;
use zero2prod::get_configuration;

#[tokio::main]
async fn main() -> Result<(), IoError> {
    let settings = get_configuration().expect("Failed to get settings");
    let address = format!("0.0.0.0:{}", settings.port);
    let listener = std::net::TcpListener::bind(&address).expect("Failed to bind to address");
    let dbpool = PgPool::connect(&settings.database.connection_string())
        .await
        .expect("Failed to create db connection pool");
    zero2prod::run(listener, dbpool)?.await
}
