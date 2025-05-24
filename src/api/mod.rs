use std::net::SocketAddr;

use axum::{extract::Path, http::{HeaderMap, HeaderValue, Method}, response::{Html, IntoResponse}, routing::get, Router};
use tower_http::cors::CorsLayer;

async fn main_page() -> Html<&'static str>{
    Html(include_str!("../../web/index.html"))
}

async fn favicon() -> &'static [u8] {
    include_bytes!("../../web/assets/icons/favicon.ico")
}

async fn load_asset(path: &str) -> String {
    let file = std::fs::read_to_string(format!("./web/assets/{}", path))
        .unwrap_or_else(|_| panic!("Failed to load asset: {}", format!("./web/assets/{}", path)));

    file
}

async fn assets(Path(path): Path<String>) -> impl IntoResponse {
    let mut headers = HeaderMap::new();

    let file_type = path.rsplit('.').next();
// Remove leading slash if present
    match file_type {
        Some("css") => {
            headers.insert("Content-Type", HeaderValue::from_static("text/css"));
        },
        Some("js") => {
            headers.insert("Content-Type", HeaderValue::from_static("application/javascript"));
        }
        _ => {
            headers.insert("Content-Type", HeaderValue::from_static("text/plain"));
        }
    }

    let response = (headers, load_asset(path.as_str()).await);

    response.into_response()
}

pub fn get_router(addr: &SocketAddr) -> Router{
    let cors = CorsLayer::new()
            .allow_origin(addr.to_string().parse::<HeaderValue>().unwrap())
            .allow_methods([Method::GET, Method::POST]);

    let router = Router::new()
                        .route("/", get(main_page))
                        .route("/favicon.ico", get(favicon))
                        .route("/assets/{path}", get(assets))
                        .layer(cors);

    router
}