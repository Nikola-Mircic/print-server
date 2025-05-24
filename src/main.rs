use std::net::SocketAddr;

use axum_server::*;

mod api;
mod error;
mod config;

use config::*;

#[tokio::main]
async fn main() {
    let addr = config().SERVER_ADDR.parse::<SocketAddr>()
                            .unwrap_or_else(|_| {
                                panic!("Invalid address format: {}", config().SERVER_ADDR);
                            });

    println!("Listening on http://{}", addr);

    let router = api::get_router(&addr);

    let handle = Handle::new();

    let server = axum_server::bind(addr)
        .handle(handle.clone())
        .serve(router.into_make_service());

    tokio::spawn(server);

    while let Some(line) = std::io::stdin().lines().next() {
        let input = line.unwrap();

        if input.trim() == "stop" {
            break;
        }
    }

    println!("Shutting down server...");
    handle.shutdown();
}
