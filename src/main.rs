use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[async_std::main]
async fn main() -> Result<(), tide::Error> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // panic if we can't fetch the configuration
    let configuration = get_configuration().expect("Failed to read configuration file.");

    // fetch port from configuration file
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address).expect("Couldn't bind port");

    // create database connection pool
    let pg_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.database.with_db());

    // Start the server
    run(listener, pg_pool).await?;
    Ok(())
}
