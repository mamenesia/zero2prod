use std::net::TcpListener;
use zero2prod::{configuration, run};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // get configuration from environment
    let configuration = configuration::get_configuration().expect("Failed to read configuration.");

    // get port and add to address
    let address = format!("127.0.0.1:{}", configuration.application_port);

    // setup listener
    let listener = TcpListener::bind(address)?;

    // run server
    run(listener)?.await
}
