use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;

use newsletter::configuration::get_configuration;
use newsletter::startup::run;
use newsletter::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("newsletter".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let config = get_configuration().expect("Failed to read configuration");

    let connection = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy(&config.database.connection_string().expose_secret())
        .expect("Failed to create Postgres connection pool.");

    let address = format!("{}:{}", config.application.host, config.application.port);

    let listener = TcpListener::bind(address)?;

    run(listener, connection)?.await
}
