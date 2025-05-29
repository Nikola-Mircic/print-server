use std::{fs, io::Write};

use axum::{extract::{Multipart, Path, State}, http::{HeaderMap, HeaderValue}, response::{Html, IntoResponse, Redirect}, Json};

use crate::{config, error::*, printer::{Printer, PrinterManager}};

use super::state::AppState;

pub async fn main_page() -> Html<&'static str>{
    Html(include_str!("../../web/index.html"))
}

pub async fn favicon() -> &'static [u8] {
    include_bytes!("../../web/assets/icons/favicon.ico")
}

async fn load_asset(path: &str) -> String {
    let file = std::fs::read_to_string(format!("./web/assets/{}", path))
        .unwrap_or_else(|_| panic!("Failed to load asset: {}", format!("./web/assets/{}", path)));

    file
}

pub async fn assets(Path(path): Path<String>) -> impl IntoResponse {
    let mut headers = HeaderMap::new();

    let file_type = path.rsplit('.').next();

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

pub async fn printers() -> impl IntoResponse{
    let manager = PrinterManager::new();

    Json(manager.list_printers()).into_response()
}

pub async fn file_receiver(State(state): State<AppState<impl Printer>>, mut file: Multipart) -> impl IntoResponse{
    while let Some(field) = file.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();  

        store_file(&name, data.to_vec()).unwrap_or_else(|e| {
            println!("Error storing file: {}", e);
        });

        state.printer().print(name.clone()).unwrap_or_else(|e| {
            println!("Error printing file: {}", e);
        });
    }
        
    Redirect::to("/")
}

fn store_file(name: &String, data: Vec<u8>) -> Result<()> {
    let dir_path = config().UPLOAD_DIR.clone();
    let file_path = format!("{}/{}", dir_path, name);

    println!("Storing file at: {}", file_path);

    fs::create_dir_all(&dir_path).map_err(|_| {
        Error::DirCreateError(dir_path.clone())
    })?;

    let mut file = fs::File::create(&file_path).map_err(|_| {
        Error::FileCreateError(file_path.clone())
    })?;

    file.write_all(&data).map_err(|_| {
        Error::FileWriteError(file_path.clone())
    })?;

    Ok(())
}