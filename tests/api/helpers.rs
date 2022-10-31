use once_cell::sync::Lazy;
use sqlx::{Connection, Executor, PgConnection, PgPool};
// use std::net::TcpListener;
use uuid::Uuid;

use newsletter::{
    configuration::{get_configuration, DatabaseSettings},
    // email_client::EmailClient,
    startup::{get_connection_pool, Application},
    telemetry::{get_subscriber, init_subscriber},
};

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();

    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    }
});

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    Lazy::force(&TRACING);

    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to Postgres");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database");

    let connection_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres");

    // Running Migration
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}

pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    let config = {
        let mut c = get_configuration().expect("Failed to read configuration");
        // Use a differnt database for each test case
        c.database.database_name = Uuid::new_v4().to_string();
        // Use a random OS port
        c.application.port = 0;
        c
    };

    configure_database(&config.database).await;

    let application = Application::build(config.clone())
        .await
        .expect("Failed to build application");

    let address = format!("http://127.0.0.1:{}", application.port());
    let _ = tokio::spawn(application.run_until_stopped());

    TestApp {
        address,
        db_pool: get_connection_pool(&config.database),
    }

    // let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // let port = listener.local_addr().unwrap().port();
    // let address = format!("http://127.0.0.1:{}", port);

    // let mut config = get_configuration().expect("Failed to read configuration");
    // config.database.database_name = Uuid::new_v4().to_string();

    // let connection_pool = configure_database(&config.database).await;

    // let sender_email = config
    //     .email_client
    //     .sender()
    //     .expect("Invaild sender email address");

    // let timeout = config.email_client.timeout();

    // let email_client = EmailClient::new(
    //     config.email_client.base_url,
    //     sender_email,
    //     config.email_client.auth_token,
    //     timeout,
    // );

    // let server = newsletter::startup::run(listener, connection_pool.clone(), email_client)
    //     .expect("Failed to bind address");

    // let _ = tokio::spawn(server);

    // TestApp {
    //     address,
    //     db_pool: connection_pool,
    // }
}
