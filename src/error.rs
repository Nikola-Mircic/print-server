use std::fmt::Display;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error{
    ConfigParseError(String),

    // Server errors
    ServerStartError(String),

    // File system errors
    DirCreateError(String),
    FileCreateError(String),
    FileWriteError(String),

    // Printer errors
    PrinterNotFound(String),
    PrintFailed(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ConfigParseError(config_name) => write!(f, "Failed to parse configuration for: {}", config_name),
            Error::DirCreateError(path) => write!(f, "Failed to create directory: {}", path),
            Error::FileCreateError(path) => write!(f, "Failed to create file: {}", path),
            Error::FileWriteError(path) => write!(f, "Failed to write to file: {}", path),
            Error::PrinterNotFound(printer) => write!(f, "Printer not found: {}", printer),
            Error::PrintFailed(reason) => write!(f, "Print operation failed: {}", reason),
            Error::ServerStartError(reason) => write!(f, "Failed to start server: {}", reason),
        }
    }
}