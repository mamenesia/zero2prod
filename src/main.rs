use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::{configuration, run};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // get configuration from environment
    let configuration = configuration::get_configuration().expect("Failed to read configuration.");

    // connect to database
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    // get port and add to address
    let address = format!("127.0.0.1:{}", configuration.application_port);
    println!("Listening on {}", address);

    // setup listener
    let listener = TcpListener::bind(address)?;

    // run server
    run(listener, connection_pool)?.await
}
