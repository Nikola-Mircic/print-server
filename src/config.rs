use std::{path::PathBuf, sync::OnceLock};

use crate::error::{Error, Result};

pub fn config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        Config::load_from_env().unwrap_or_else(|err| {
            panic!("Failed to load configuration: {}", err);
        })
    })
}

#[allow(non_snake_case)]
pub struct Config {
    pub SERVER_ADDR: String,
    pub UPLOAD_DIR: String,
}

impl Config{
    fn load_from_env() -> Result<Self> {
        let server_port = get_env_as::<u16>("SERVER_PORT")?;
        let server_host= get_env("SERVER_HOST")?;
        
        let upload_dir = get_env("UPLOAD_DIR").unwrap_or_else(|_| "./upload".to_string());

        Ok(Config { 
            SERVER_ADDR: format!("{}:{}", server_host, server_port),
            UPLOAD_DIR: PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(upload_dir).to_string_lossy().to_string(),
        })
    }
}

fn get_env(name: &str) -> Result<String> {
    std::env::var(name).map_err(|_| Error::ConfigParseError(name.to_string()))
}

fn get_env_as<T>(name: &str) -> Result<T>
where
    T: std::str::FromStr,
{
    let value = get_env(name)?;
    value.parse::<T>()
        .map_err(|_| Error::ConfigParseError(name.to_string()))
}