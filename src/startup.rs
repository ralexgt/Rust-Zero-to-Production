use std::net::TcpListener;

use actix_web::{App, HttpServer, dev::Server, web};

use crate::routes::{health, subscribe};

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health))
            .route("subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
