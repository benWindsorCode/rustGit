use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Config {
    path: String,
    pub contents: ConfigContents
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigContents {
    pub core: CoreContents
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoreContents {
    pub repository_format_version: i8,
    pub filemode: bool,
    pub bare: bool
}

impl Config {
    pub fn new(path: String) -> Self {
        let config_contents = ConfigContents {
            core: CoreContents::new()
        };

        Config { path, contents: config_contents }
    }

    pub fn write(&self) -> Result<(), String> {
        let config_json = serde_json::to_string(&self.contents).or_else(|e| Err(e.to_string()))?;
        fs::write(&self.path, config_json).or_else(|e| Err(e.to_string()))
    }

    pub fn read(&mut self) -> Result<(), String> {
        let data = match fs::read_to_string(&self.path) {
            Ok(data) => data,
            Err(_) => return Err("Failed to read config".to_string())
        };

        serde_json::from_str(data.as_str()).and_then(|contents| {
            self.contents = contents;
            Ok(())
        }).or_else(|e| Err(e.to_string()))
    }
}

impl CoreContents {
    pub fn new() -> Self {
        CoreContents {
            repository_format_version: 0,
            filemode: false,
            bare: false
        }
    }
}