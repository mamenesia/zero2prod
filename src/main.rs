use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use zero2prod::{
    configuration, run,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Redirect all `log`'s events to our subscriber
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // get configuration from environment
    let configuration = configuration::get_configuration().expect("Failed to read configuration.");

    // connect to database
    let connection_pool =
        PgPoolOptions::new().connect_lazy_with(configuration.database.connection_options());

    // get port and add to address
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    println!("Listening on {}", address);

    // setup listener
    let listener = TcpListener::bind(address)?;

    // run server
    run(listener, connection_pool)?.await
}
