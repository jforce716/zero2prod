use actix_web::{HttpResponse, web};
use sqlx::PgPool;
use uuid::{Uuid};

#[derive(serde::Deserialize)]
pub struct Subscriber {
    pub name: String,
    pub email: String,
}

pub async fn subscribe(
    subscriber: web::Form<Subscriber>, conn_pool: web::Data<PgPool>,
) -> HttpResponse {
    let result = sqlx::query!(r#"
        INSERT INTO subscription(id, email, name, subscribed_at) VALUES($1, $2, $3, $4)
        "#,
        Uuid::now_v7(),
        subscriber.email,
        subscriber.name,
        chrono::Utc::now()
    )
    .execute(conn_pool.get_ref())
    .await;
    if let Err(e) = result {
        eprintln!("Failed to insert subscriber information: {}", e);
        return HttpResponse::InternalServerError().finish();
    }
    HttpResponse::Ok().finish()
}