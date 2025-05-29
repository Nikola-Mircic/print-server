use std::net::SocketAddr;

use axum::{http::{HeaderValue, Method}, 
           routing::{get, post}, 
           Router};

use tower_http::cors::CorsLayer;

use crate::printer::Printer;

mod routh_methods;
mod state;

use routh_methods::*;
use state::*;

pub fn get_router<T: Printer>(addr: &SocketAddr, printer: T) -> Router{
    let cors = CorsLayer::new()
            .allow_origin(addr.to_string().parse::<HeaderValue>().unwrap())
            .allow_methods([Method::GET, Method::POST]);

    println!("Generating app state with printer: {:?}", printer);
    let state = AppState::new(printer);

    println!("Creating router...");
    let router = Router::new()
                        .route("/", get(main_page))
                        .route("/favicon.ico", get(favicon))
                        .route("/assets/{path}", get(assets))
                        .route("/printers", get(printers))
                        .route("/print", post(file_receiver))
                        .layer(cors)
                        .with_state(state);

    router
}