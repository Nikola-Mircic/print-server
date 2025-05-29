use std::{io::Write, net::SocketAddr, path::PathBuf};

use axum_server::*;

mod api;
mod error;
mod config;
mod printer;

use config::*;
use printer::*;
use error::*;
use api::*;

async fn init_server(addr: SocketAddr) -> (axum_server::Server, Handle) {
    let handle = Handle::new();

    let server = axum_server::bind(addr)
        .handle(handle.clone());

    (server, handle)
}

async fn wait_for_shutdown(handle: Handle) {
    while let Some(line) = std::io::stdin().lines().next() {
        let input = line.unwrap();

        if input.trim() == "stop" {
            break;
        }
    }

    println!("Shutting down server...");
    handle.shutdown();
}

fn create_printer_manager() -> Result<PrinterManager> {
    let mut manager = PrinterManager::new();
    
    let printers = manager.list_printers();

    println!("Please select a printer by number:");

    if printers.is_empty() {
        return Err(Error::PrinterNotFound("No printers found".to_string()));
    } else {
        println!("Available printers:");
        for (i, printer) in printers.iter().enumerate() {
            println!("{}: {}", i + 1, printer);
        }
    }

    let mut input = String::new();
    std::io::stdout().write_all(b"Enter printer number: ").unwrap();
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input).unwrap();

    let selected_printer_index: usize = input.trim().parse().unwrap_or(0);

    manager.set_default_printer(selected_printer_index - 1);

    println!("Selected printer: {:?}", manager.default_printer());

    Ok(manager)
}

#[tokio::main]
async fn main() -> Result<()> {
    /*let addr = config().SERVER_ADDR.parse::<SocketAddr>()
        .unwrap_or_else(|_| {
            panic!("Invalid address format: {}", config().SERVER_ADDR);
        });*/

    let addr = config().SERVER_ADDR.parse::<SocketAddr>()
        .map_err(|_| {
            Error::ConfigParseError("Invalid SERVER_ADDR format".to_string())
        })?;
    
    let printer_manager = create_printer_manager().unwrap();

    let default_printer = printer_manager.default_printer()
        .ok_or_else(|| Error::PrinterNotFound("No default printer set".to_string()))
        .unwrap();

    let upload_dir = config().UPLOAD_DIR.clone();

    let printer = WinPrinter{
        device: default_printer.to_owned(), 
        folder_path: PathBuf::from(upload_dir)
    };

    let router = get_router(&addr, printer);

    let (server, handle) = init_server(addr).await;

    println!("Listening on http://{}", addr);

    tokio::spawn(wait_for_shutdown(handle));

    println!("Starting server...");
    server.serve(router.into_make_service()).await.map_err(|e| {
        eprintln!("Server error: {}", e);
        Error::ServerStartError(e.to_string())
    })
}


