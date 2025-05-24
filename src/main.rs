use std::net::SocketAddr;

use axum_server::*;

mod api;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 1653));
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
