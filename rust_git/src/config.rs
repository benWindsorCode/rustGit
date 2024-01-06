use std::fs;
use serde::{Deserialize, Serialize};
use crate::repository::Repository;
use crate::utils::repo_file;

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

    pub fn write(&self) -> Result<(), &'static str> {
        let config_json = serde_json::to_string(&self.contents).unwrap();
        fs::write(&self.path, config_json).or_else(|_| Err("Failed to write to config file"))
    }

    pub fn read(&mut self) -> Result<(), &'static str> {
        // fs::read_to_string(&self.path).and_then(|data| serde_json::from_str(data.as_str())?).or_else(|_| Err("Failed to read config file"))
        todo!("Implement config read")
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