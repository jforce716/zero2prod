use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::{App, HttpServer, web};
use sqlx::PgPool;
use std::io::Error as IoError;
use std::net::TcpListener;

pub fn run(listener: TcpListener, conn_pool: PgPool) -> Result<Server, IoError> {
    let pool = web::Data::new(conn_pool);
    let server = HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .route("/healthcheck", web::get().to(health_check))
            .route("/subscribe", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
