use std::net::TcpListener;

// use sqlx::{Connection, PgConnection};
use zero2prod::{configuration::get_configuration, startup::run};

#[tokio::test]
async fn check_health_success() {
    let address = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health", address))
        .send()
        .await
        .expect("Failed to execute a request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_200_for_valid_req() {
    let address = spawn_app().await;
    // let configuration = get_configuration().expect("Config should be in root");
    // let connection_string = configuration.database.connection_string();
    // let mut db_connection = PgConnection::connect(&connection_string)
    // .await
    // .expect("Failed to connect to local database");
    let client = reqwest::Client::new();

    let body = "name=Le%20Guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(format!("{}/subscriptions", address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(200, response.status().as_u16());

    // let saved = sqlx::query!("SELECT email, name FROM subscriptions")
    // .fetch_one(&mut db_connection)
    // .await
    // .expect("Failed to fetch the subscription in test");

    // assert_eq!(saved.name, "Le Guin");
    // assert_eq!(saved.email, "ursula_le_guin@gmail.com");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let address = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(format!("{}/subscriptions", address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}

async fn spawn_app() -> String {
    let configuration = get_configuration().expect("Configuration missing");
    // :0 assigns a random free port
    let listener = TcpListener::bind(format!("{}:0", configuration.host))
        .expect("Failed to bind on random port");
    // Save the port to return it later
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to bind address");
    _ = tokio::spawn(server);

    format!("http://{}:{port}", configuration.host)
}
