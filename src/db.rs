use dirs::home_dir;
use std::{error::Error, io::ErrorKind};

use serde::{Deserialize, Serialize};
use tokio::fs::{read_to_string, write};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub api_key: Option<String>,
    pub notes_folder: String,
}

pub async fn get_config() -> Result<Config, Box<dyn Error>> {
    let mut config_file_location = home_dir().unwrap();
    config_file_location.push(".gptnotes.json");
    let file = match read_to_string(&config_file_location).await {
        Ok(config) => config,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                let config = Config {
                    api_key: None,
                    notes_folder: "./".to_string(),
                };
                let config = serde_json::to_string(&config)?;
                write(config_file_location, config.clone()).await?;
                config
            }
            _ => panic!("Error reading config file: {}", error),
        },
    };

    let config: Config = serde_json::from_str(file.as_str()).unwrap();

    Ok(config)
}
