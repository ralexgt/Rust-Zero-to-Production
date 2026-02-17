use std::net::TcpListener;

use zero2prod::{configuration::get_configuration, startup};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("No configuration file");
    let address = format!("{}:{}", configuration.host, configuration.port);
    let listener = TcpListener::bind(address)?;
    startup::run(listener)?.await
}
