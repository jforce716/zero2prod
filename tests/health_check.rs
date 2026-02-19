use sqlx::PgPool;

fn start_server(pool: PgPool) -> String {
    let listener = std::net::TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener, pool).unwrap();
    tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

#[sqlx::test]
async fn test_health_check(pool: PgPool) {
    let address = start_server(pool);
    let client = reqwest::Client::new();
    let target_url = format!("{}/healthcheck", address);

    let response = client.get(&target_url).send().await.expect("Request failed");
    assert_eq!(200, response.status().as_u16());
}

#[sqlx::test]
async fn test_subscribe_with_valid_form_data(pool: PgPool) {
    let address = start_server(pool.clone());
    let client = reqwest::Client::new();

    let form_data = "name=Jun%20Tan&email=jforce716%40gmail.com";
    let target_url = format!("{}/subscribe", address);
    let response = client.post(&target_url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(form_data)
        .send().await.expect("Request failed");
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscription where email=$1", "jforce716@gmail.com")
        .fetch_one(&pool)
        .await
        .expect("Failed to fetch saved subscription");
    assert_eq!(saved.email, "jforce716@gmail.com");
    assert_eq!(saved.name, "Jun Tan");
}

#[sqlx::test]
async fn test_subscribe_with_invalid_data(pool: PgPool) {
    let address = start_server(pool);
    let client = reqwest::Client::new();

    let target_url = format!("{}/subscribe", address);
    let test_data = vec![
        ("missing email", "name=Test%20User"),
        ("mission name", "email=somename%40company.com"),
        ("missing both name and email", "")
    ];
    for (error_message, body) in &test_data {
        if let Ok(response) = client.post(&target_url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(*body)
            .send()
            .await {
            assert_eq!(400, response.status().as_u16(), 
                "he API did not fail with 400 Bad request when payload was {}",
                *error_message);
        } else {
            eprint!("Failed to execute request test test {}", *error_message)
        }
    }
}